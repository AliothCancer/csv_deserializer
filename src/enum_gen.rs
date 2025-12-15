

macro_rules! create_enum {
    // --- 1. Caso Base: Input finito ---
    // Quando la lista di input `[]` è vuota, generiamo il codice finale.
    (@step $name:ident, ($($variants:ident),*), ($($arms:tt)*), []) => {
        use std::str::FromStr;
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $name {
            $($variants),*
        }

        impl FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    // Inseriamo qui tutti i rami match accumulati
                    $($arms)*
                    _ => Err(format!("Valore non riconosciuto: '{}'", s)),
                }
            }
        }
    };

    // --- 2. Caso Complesso: "str1" | "str2" => Variante ---
    // Riconosce il pattern stringa/e => Identificatore
    (@step $name:ident, ($($vars:ident),*), ($($arms:tt)*), [ $($l:literal)|+ => $v:ident, $($rest:tt)* ]) => {
        create_enum!(
            @step 
            $name, 
            ($($vars,)* $v), // Aggiunge la variante alla lista
            ($($arms)* $($l)|+ => Ok($name::$v),), // Aggiunge il match arm personalizzato
            [ $($rest)* ] // Continua con il resto
        );
    };
    // Gestione dell'ultimo elemento (senza virgola finale) per il caso complesso
    (@step $name:ident, ($($vars:ident),*), ($($arms:tt)*), [ $($l:literal)|+ => $v:ident ]) => {
        create_enum!(@step $name, ($($vars,)* $v), ($($arms)* $($l)|+ => Ok($name::$v),), []);
    };

    // --- 3. Caso Semplice: Variante ---
    // Riconosce solo l'Identificatore (usa il nome stesso come stringa)
    (@step $name:ident, ($($vars:ident),*), ($($arms:tt)*), [ $v:ident, $($rest:tt)* ]) => {
        create_enum!(
            @step 
            $name, 
            ($($vars,)* $v), 
            ($($arms)* stringify!($v) => Ok($name::$v),), 
            [ $($rest)* ]
        );
    };
    // Gestione dell'ultimo elemento (senza virgola finale) per il caso semplice
    (@step $name:ident, ($($vars:ident),*), ($($arms:tt)*), [ $v:ident ]) => {
        create_enum!(@step $name, ($($vars,)* $v), ($($arms)* stringify!($v) => Ok($name::$v),), []);
    };

    // --- Entry Point ---
    // Inizializza gli accumulatori vuoti
    ($name:ident; $($input:tt)*) => {
        create_enum!(@step $name, (), (), [ $($input)* ]);
    };
}
