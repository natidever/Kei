// use anyhow::Result;
// use askama::Template;
// use python_ast::{self as ast};
// use std::fs;
// use std::path::PathBuf;

// #[derive(Debug, Clone)]
// struct ClassInfo {
//     name: String,
//     fields: Vec<Field>,
// }

// #[derive(Debug, Clone)]
// struct Field {
//     name: String,
//     typ: String,
//     optional: bool,
//     default: Option<String>,
// }

// #[derive(Template)]
// #[template(path = "../templates/user_create.py.j2", escape = "none")]
// struct UserCreateTemplate {
//     class_name: String,
//     fields: Vec<Field>,
// }

// #[derive(Template)]
// #[template(path = "../templates/user_update.py.j2", escape = "none")]
// struct UserUpdateTemplate {
//     class_name: String,
//     fields: Vec<Field>,
// }

// #[derive(Template)]
// #[template(path = "../templates/user_response.py.j2", escape = "none")]
// struct UserResponseTemplate {
//     class_name: String,
//     fields: Vec<Field>,
// }

// pub struct Generator {
//     input_path: PathBuf,
//     output_path: PathBuf,
// }

// impl Generator {
//     pub fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
//         Self {
//             input_path,
//             output_path,
//         }
//     }

//     pub fn generate(&self) -> Result<()> {
//         let mut visitor = ClassVisitor::new();

//         // Parse Python files
//         for entry in walkdir::WalkDir::new(&self.input_path) {
//             let entry = entry?;
//             if entry.file_type().is_file() && entry.path().extension() == Some("py".as_ref()) {
//                 let content = fs::read_to_string(entry.path())?;
//                 let module = ast::parse(&content)?;
//                 visitor.visit(&module);
//             }
//         }

//         // Generate code for each class
//         for class_info in visitor.classes() {
//             self.generate_class(&class_info)?;
//         }

//         Ok(())
//     }

//     fn generate_class(&self, class_info: &ClassInfo) -> Result<()> {
//         // Create templates
//         let create_template = UserCreateTemplate {
//             class_name: format!("{}Create", class_info.name),
//             fields: class_info
//                 .fields
//                 .iter()
//                 .filter(|f| f.name != "id")
//                 .cloned()
//                 .collect(),
//         };

//         let update_template = UserUpdateTemplate {
//             class_name: format!("{}Update", class_info.name),
//             fields: class_info
//                 .fields
//                 .iter()
//                 .map(|f| Field {
//                     optional: true,
//                     ..f.clone()
//                 })
//                 .collect(),
//         };

//         let response_template = UserResponseTemplate {
//             class_name: format!("{}Response", class_info.name),
//             fields: class_info.fields.clone(),
//         };

//         // Write files
//         fs::write(
//             self.output_path
//                 .join(format!("{}_create.py", class_info.name.to_lowercase())),
//             create_template.render()?,
//         )?;
//         fs::write(
//             self.output_path
//                 .join(format!("{}_update.py", class_info.name.to_lowercase())),
//             update_template.render()?,
//         )?;
//         fs::write(
//             self.output_path
//                 .join(format!("{}_response.py", class_info.name.to_lowercase())),
//             response_template.render()?,
//         )?;

//         Ok(())
//     }
// }

// struct ClassVisitor {
//     classes: Vec<ClassInfo>,
//     current_class: Option<String>,
//     current_fields: Vec<Field>,
//     current_class_name: String,
// }

// impl Default for ClassVisitor {
//     fn default() -> Self {
//         Self {
//             classes: Vec::new(),
//             current_class: None,
//             current_fields: Vec::new(),
//             current_class_name: String::new(),
//         }
//     }
// }

// impl ClassVisitor {
//     fn new() -> Self {
//         Self {
//             classes: Vec::new(),
//             current_class: None,
//             current_fields: Vec::new(),
//         }
//     }

//     fn classes(&self) -> &[ClassInfo] {
//         &self.classes
//     }

//     fn visit(&mut self, node: &ast::Node) {
//         match node {
//             ast::Node::Stmt(ast::Stmt::ClassDef {
//                 name,
//                 body,
//                 decorator_list,
//                 ..
//             }) => {
//                 if let Some(decorator) = decorator_list.first() {
//                     if let ast::Node::Expr(ast::Expr::Name { id, .. }) = decorator {
//                         if id == "generate" {
//                             self.current_class = Some(name.clone());
//                             self.current_class_name = name.clone();
//                         }
//                     }
//                 }

//                 for stmt in body {
//                     self.visit(stmt);
//                 }

//                 if let Some(class_name) = self.current_class.take() {
//                     self.classes.push(ClassInfo {
//                         name: class_name,
//                         fields: std::mem::take(&mut self.current_fields),
//                     });
//                 }
//             }
//             ast::Node::Stmt(ast::Stmt::Assign { targets, value, .. }) => {
//                 if let Some(name) = targets.first() {
//                     if let ast::Node::Expr(ast::Expr::Name { id, .. }) = name {
//                         if let ast::Node::Expr(ast::Expr::Call {
//                             func,
//                             args,
//                             keywords,
//                         }) = value
//                         {
//                             if let ast::Node::Expr(ast::Expr::Name { id: typ, .. }) = func {
//                                 let optional = keywords.iter().any(|k| {
//                                     k.arg.as_ref() == Some("optional")
//                                         && match k.value {
//                                             ast::Node::Expr(ast::Expr::Constant {
//                                                 value: ast::Constant::Bool(true),
//                                                 ..
//                                             }) => true,
//                                             _ => false,
//                                         }
//                                 });
//                                 let default = keywords.iter().find_map(|k| {
//                                     if k.arg.as_ref() == Some("default") {
//                                         Some(match k.value {
//                                             ast::Node::Expr(ast::Expr::Constant {
//                                                 value, ..
//                                             }) => value.to_string(),
//                                             _ => "None".to_string(),
//                                         })
//                                     } else {
//                                         None
//                                     }
//                                 });

//                                 self.current_fields.push(Field {
//                                     name: id.clone(),
//                                     typ: typ.clone(),
//                                     optional,
//                                     default,
//                                 });
//                             }
//                         }
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
// }
