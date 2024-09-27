/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* main.rs                                                                                 :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/21 15:18:07 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/21 15:18:15 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

mod password_generator;
use password_generator::{pool::Pool, PasswordGenerator};

fn main() {
    let pwgen: PasswordGenerator = PasswordGenerator::new()
        .pool(Pool::UPPERCASE | Pool::LOWERCASE | Pool::NUMBER | Pool::SYMBOL)
        .length(8);
    let password: String = pwgen.finalize();
    println!("{}", password);
}
