#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: String,
}

const DNA: [char; 4] = [ 'G', 'C', 'T', 'A' ];
const RNA: [char; 4] = [ 'C', 'G', 'A', 'U' ];

fn validate(s: &str, chars: [char; 4]) -> Result<String, usize> {
    match s.chars().position(|c| !chars.contains(&c)) {
        Some(x) => Err(x),
        None => Ok(s.to_string())
    }
}


impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, DNA).map(|nucleotides| Dna { nucleotides })

        // todo!("Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
    }

    pub fn into_rna(self) -> Rna {
        let s = self.nucleotides.chars().map(|c| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => panic!("invaild dna!"),
        }).collect::<String>();
        Rna::new(&s).unwrap()
        // todo!("Transform Dna {self:?} into corresponding Rna");
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, RNA).map(|nucleotides| Rna { nucleotides })
        // todo!("Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
    }
}
