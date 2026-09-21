//! Variabili d'ambiente: `ALTKEEPER_<NOME>`, con `ALTREFRESH_<NOME>` come ripiego.
//!
//! Il progetto si chiamava altrefresh: chi ha già un `.env` o un compose con i vecchi nomi
//! non deve toccare niente. Se ci sono entrambe vince la nuova; una variabile vuota conta
//! come non impostata.

/// Legge `ALTKEEPER_<name>` e, se manca o è vuota, `ALTREFRESH_<name>`.
pub fn var(name: &str) -> Option<String> {
    pick(
        std::env::var(format!("ALTKEEPER_{name}")).ok(),
        std::env::var(format!("ALTREFRESH_{name}")).ok(),
    )
}

/// Vero se la variabile è impostata (anche a un valore vuoto) con uno dei due nomi.
pub fn is_set(name: &str) -> bool {
    std::env::var_os(format!("ALTKEEPER_{name}")).is_some()
        || std::env::var_os(format!("ALTREFRESH_{name}")).is_some()
}

fn pick(new: Option<String>, old: Option<String>) -> Option<String> {
    new.filter(|v| !v.is_empty())
        .or_else(|| old.filter(|v| !v.is_empty()))
}

#[cfg(test)]
mod tests {
    use super::pick;

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    #[test]
    fn il_nome_nuovo_vince() {
        assert_eq!(pick(s("nuovo"), s("vecchio")), s("nuovo"));
    }

    #[test]
    fn il_nome_vecchio_funziona_ancora() {
        assert_eq!(pick(None, s("vecchio")), s("vecchio"));
    }

    #[test]
    fn un_valore_vuoto_conta_come_assente() {
        assert_eq!(pick(s(""), s("vecchio")), s("vecchio"));
        assert_eq!(pick(s(""), s("")), None);
        assert_eq!(pick(None, None), None);
    }
}
