rez::rez! {
    používej standardní_knihovna::{fs, io};
    používej standardní_knihovna::io::{Read, Write};

    veřejná funkce hlavní() {
        jsou měnitelné argumenty = vec![];
        pro argument ve standardní_knihovna::env::args().přeskoč(1) {
            argumenty.vtlač(argument);
        }
        jestli argumenty.délka() == 0 {
            argumenty.vtlač(Řetězec::z("-"));
        }

        pro argument v argumenty {
            je obsah: Řetězec;
            když argument == "-" {
                je měnitelný nárazník: Vektor<u8> = vec![];
                je měnitelný standartní_vstup = io::stdin();
                standartní_vstup.read_to_end(&mut nárazník).rozbal();
                obsah = Řetězec::from_utf8_lossy(&nárazník).na_řetězec();
            } jinak {
                obsah = fs::read_to_string(&argument).rozbal();
            }
            io::stdout().write_all(obsah.jako_bajty()).rozbal();
        }
    }
}
