/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* store.rs                                                                                :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/10/09 20:43:12 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/10/09 20:43:12 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use anyhow::{Ok, Result};

pub(in crate::password_generator) fn store(pw: &str) -> Result<()> {
    let path: &Path = Path::new("password.txt");
    if !path.is_file() {
        let mut file: File = File::create(path.to_str().unwrap())?;
        file.write_all(format!("{}\n", pw).as_bytes())?;
        Ok(())
    } else {
        let mut file: File = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path.to_str().unwrap())?;
        file.write_all(format!("{}\n", pw).as_bytes())?;
        Ok(())
    }
}
