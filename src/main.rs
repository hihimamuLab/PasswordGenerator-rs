/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* main.rs                                                                                 :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/21 15:18:07 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/27 21:14:44 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

mod password_generator;
use password_generator::{pool::Pool, PasswordGenerator};

fn main() {
    let pasword_genrator: PasswordGenerator = PasswordGenerator::new()
        .pool(Pool::UPPERCASE | Pool::LOWERCASE | Pool::NUMBER | Pool::SYMBOL)
        .length(8);
    let password: String = pasword_genrator.finalize();
    println!("{}", password);
}
