use std::time::{ SystemTime, UNIX_EPOCH };
use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };
use std::env;

struct NahodaGenerator {
    state: u128,
    counter: u64,
    tajemstvi: [u8; 16],
}

impl NahodaGenerator {
    /// vytvori novy NahodaGenerator se stavem inicializovanym z maximalne mozneho poctu zdroju entropie
    fn new() -> Self {
        // ziskani entropie ze systemoveho casu s nanosekundovou presnosti
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

        // ziskani entropie z adres v pameti (velmi nepredvidatelne)
        let addr1 = &now as *const u128 as usize;
        let addr2 = &addr1 as *const usize as usize;

        // michani entropie neobvyklym zpusobem
        let mut state = now ^ (addr1 as u128) ^ ((addr2 as u128) << 37);

        // pouziti ID procesu jako dodatecne entropie
        state = state ^ (std::process::id() as u128);

        // pouziti informace o vlakne jako dodatecne entropie (bez pouziti nestabilniho API)
        let thread_ptr = std::thread::current();
        let thread_addr = &thread_ptr as *const _ as usize;
        state = state ^ (thread_addr as u128);

        // pouziti nahodnych bitu z adresar environmentalnich promennych
        for (key, val) in env::vars() {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            val.hash(&mut hasher);
            state = state ^ (hasher.finish() as u128);
        }

        // pouziti adresy zasobniku jako zdroje entropie
        let stack_var = 0u8;
        let stack_addr = &stack_var as *const u8 as usize;
        state = state ^ (stack_addr as u128);

        // pouziti velikosti pameti
        let allocation = Box::new([0u8; 16]);
        let heap_addr = &allocation[0] as *const u8 as usize;
        state = state ^ (heap_addr as u128);
        drop(allocation);

        // vytvoreni tajneho bufferu s podivnym michanim
        let mut tajemstvi = [0u8; 16];
        for i in 0..16 {
            tajemstvi[i] = ((state >> (i * 7)) & 0xff) as u8;
            tajemstvi[i] = tajemstvi[i].rotate_left(3).wrapping_add(i as u8);
            // michani s casem aby se porad menilo
            tajemstvi[i] = tajemstvi[i].wrapping_add(
                (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() & 0xff) as u8
            );
        }

        NahodaGenerator {
            state,
            counter: 0,
            tajemstvi,
        }
    }

    /// generuje dalsi nahodnou hodnotu u64 pomoci chaotickych operaci
    fn dalsi_cislo(&mut self) -> u64 {
        self.counter = self.counter.wrapping_add(1);

        // aktualizace stavu s prvociselnymi nasobiteli
        self.state = self.state.wrapping_mul(6364136223846793005);
        self.state = self.state.wrapping_add(1442695040888963407);

        // zamichani tajemstvi podivnym zpusobem
        for (i, &b) in self.tajemstvi.iter().enumerate() {
            if i % 3 == 0 {
                self.state = self.state ^ ((b as u128) << (i * 11) % 121);
            } else if i % 3 == 1 {
                self.state = self.state.rotate_left((b as u32) % 61);
            } else {
                self.state = self.state.wrapping_add((b as u128) << 42);
            }
        }

        // pridani aktualni cas pro jeste vetsi nahodnost
        self.state =
            self.state ^
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();

        // aplikace chaoticke bitove manipulace
        let mut x = self.state;
        x = x ^ (x >> 12);
        x = x ^ (x << 25);
        x = x ^ (x >> 27);

        // zamichani pocitadla specialnim zpusobem
        x = x ^ (self.counter as u128).rotate_left(33);

        // aplikace hashovaciho michani pro lepsi distribuci
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        let hash_val = hasher.finish();

        // finalni michani s trochou bitove magie
        let result = hash_val ^ ((x >> 64) as u64) ^ (x as u64);
        result.rotate_right(((self.counter % 63) + 1) as u32)
    }

    /// generuje nahodne cislo v danem rozsahu [min, max]
    fn cislo_v_rozsahu(&mut self, min: u64, max: u64) -> u64 {
        // kontrola, ze max je vetsi nebo roven min
        if max <= min {
            return min; // pokud je neplatny rozsah, vrat minimum
        }

        let rozsah = max - min + 1; // +1 pro zahrnutí maxima
        let nahodne = self.dalsi_cislo();

        // vypocet cisla v danem rozsahu
        min + (nahodne % rozsah)
    }
}

fn main() {
    let mut generator = NahodaGenerator::new();
    let args: Vec<String> = std::env::args().collect();

    // kontrola povinnych argumentu min a max
    if args.len() < 3 {
        println!("chyba: chybi povinne parametry min a max");
        println!("pouziti: nahoda_question-mark.exe <min> <max> [pocet]");
        println!("  <min>   - minimalni hodnota (povinne)");
        println!("  <max>   - maximalni hodnota (povinne)");
        println!("  [pocet] - pocet generovanych cisel (nepovinne, vychozi: 1, max: 100)");
        return;
    }

    // pokus o ziskani min a max hodnot z argumentu
    let min_result = args[1].parse::<u64>();
    let max_result = args[2].parse::<u64>();

    match (min_result, max_result) {
        (Ok(min), Ok(max)) => {
            // ziskani poctu cisel z volitelneho argumentu
            let mut pocet = 1; // vychozi hodnota

            if args.len() >= 4 {
                match args[3].parse::<usize>() {
                    Ok(n) if n >= 1 && n <= 100 => {
                        pocet = n;
                    }
                    Ok(n) => {
                        println!("upozorneni: pocet {} mimo rozsah, pouzivam omezeni 1-100", n);
                        pocet = n.clamp(1, 100); // omezeni na rozsah 1-100
                    }
                    Err(_) => {
                        println!("upozorneni: neplatny format poctu, pouzivam vychozi hodnotu 10");
                    }
                }
            }

            println!("nahoda - generator nahodnych cisel v rozsahu [{}, {}]", min, max);

            // generovani pozadovaneho poctu nahodnych cisel v danem rozsahu
            println!("nahodna cisla v danem rozsahu (pocet: {}):", pocet);
            for i in 1..=pocet {
                println!("cislo {}: {}", i, generator.cislo_v_rozsahu(min, max));
            }
        }
        _ => {
            println!(
                "chyba: neplatne hodnoty min nebo max. obe hodnoty musi byt kladna cela cisla."
            );
            println!("pouziti: nahoda_question-mark.exe <min> <max> [pocet]");
        }
    }
}
