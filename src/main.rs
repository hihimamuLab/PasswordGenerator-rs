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
    for _ in 0..1{
        let password_generator: PasswordGenerator = PasswordGenerator::new()
            .pool(Pool::UPPERCASE | Pool::LOWERCASE | Pool::NUMBER | Pool::SYMBOL)
            .no_similar()
        .length(12);
        let password: String = password_generator.finalize();
        println!("{}", password);
        PasswordGenerator::store(&password).unwrap();
    }
}
