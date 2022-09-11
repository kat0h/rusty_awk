/*
 * file: ast.rs
 * author: kota kato 2022
 * description:
 *   make ast/ public
 */

pub mod def;
pub mod item;
pub mod number;
pub mod print;
pub mod print_expr;
pub mod program;
pub mod string;

/*
 * Expr : AWKNUMBER | AWKSTRING
 */

/*
 *  paction : pattern '{' action '}'
 *  pattern : BEGIN
 *          | END
 *          |
 *  action  : print '(' expr ')'
 *          | print expr
*/
