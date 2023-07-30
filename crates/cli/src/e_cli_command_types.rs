pub enum ECLICommandTypes {
    HELP    = 1,
    CREATE  = 2,
    READ    = 3,
    UPDATE  = 4,
    DELETE  = 5,
}

impl ECLICommandTypes {
    pub fn from_string(s: &str) -> Self {
        match s {
            "help"   => ECLICommandTypes::HELP,
            "create" => ECLICommandTypes::CREATE,
            "read"   => ECLICommandTypes::READ,
            "UPDATE" => ECLICommandTypes::UPDATE,
            "DELETE" => ECLICommandTypes::DELETE,
            _        => ECLICommandTypes::HELP,
        }
    }
}
