use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn launch(icon_path: &str) -> io::Result<()> 
{

    // Racine : HKEY_CURRENT_USER\Software\Classes
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (classes, _) = hkcu.create_subkey("Software\\Classes")?;

    // Associe l’extension .snote à un type
    let (snote_ext, _) = classes.create_subkey(".snote")?;
    snote_ext.set_value("", &"SilverNote.File")?;

    // Crée le type de fichier
    let (snote_file, _) = classes.create_subkey("SilverNote.File")?;
    snote_file.set_value("", &"Fichier SilverNote")?;

    // Définit l’icône par défaut
    let (icon, _) = snote_file.create_subkey("DefaultIcon")?;
    icon.set_value("", &icon_path)?;

    println!("✅ Icône enregistrée pour .snote : {}", icon_path);
    Ok(())
    
}
