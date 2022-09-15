use std::io;

use crate::ast::def::*;

#[derive(Debug, PartialEq)]
struct AWKFields {
    fields: Vec<String>,
}

impl AWKFields {
    fn nf(&self) -> usize {
        self.fields.len()
    }
    fn get_field(&self, n: usize) -> Result<String, ()> {
        if n == 0 {
            Ok(self.fields.join(" "))
        } else if (1 <= n) && (n <= self.nf()) {
            Ok(self.fields[n - 1].clone())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AWKCore {
    program: AWKProgram,
    // environment
    nr: i64,
}

impl AWKCore {
    pub fn new_core(program: AWKProgram) -> AWKCore {
        return AWKCore { program, nr: 0 };
    }

    pub fn exec_program(&mut self) {
        // find BEGIN pattern
        println!("---BEGIN---");
        for i in &self.program.item_list {
            match i {
                AWKItem::AWKPatternAction(pattern_action) => {
                    match pattern_action.pattern {
                        AWKPattern::Begin => {
                            self.exec_awkaction(&pattern_action.action);
                        }
                        _ => (),
                    };
                }
            };
        }

        loop {
            self.nr += 1;
            // Read one line from stdin
            let mut line = String::new();
            if io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line.")
                != 0
            {
                let fields = AWKFields {
                    fields: line
                        .trim()
                        .split_whitespace()
                        .map(|f| f.to_string())
                        .collect(),
                };
                let nf = fields.nf();
                println!("NF: {nf}");
                /*
                for f in 0..=nf {
                    println!("${}: {}", f, fields.get_field(f).unwrap_or("".to_string()));
                }
                */
            } else {
                break;
            }
        }

        // find END pattern
        println!("---END---");
        for i in &self.program.item_list {
            match i {
                AWKItem::AWKPatternAction(pattern_action) => {
                    match pattern_action.pattern {
                        AWKPattern::End => self.exec_awkaction(&pattern_action.action),
                        _ => (),
                    };
                }
            };
        }
    }

    fn exec_awkaction(&self, actions: &Vec<AWKStatement>) {
        for action in actions.iter() {
            dbg!(&action);
        }
    }

    fn fmt_awkvalue(value: AWKValue) -> String {
        match value {
            AWKValue::AWKNumber(_n) => "hoge".to_string(),
            AWKValue::AWKString(s) => s.value.clone(),
        }
    }

    fn exec_awk_expr(&self, expr: AWKExpr) -> AWKValue {
        match expr {
            AWKExpr::AWKValue(value) => value,
        }
    }
}
