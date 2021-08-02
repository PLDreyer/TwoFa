use boringauth::oath::{ TOTPBuilder, HashFunction };
use serde_json::{Map, Value};
use std::convert::TryInto;
use std::fmt::Display;
use super::Opts;

pub enum Encoding {
    Base32,
    Hex,
    Ascii,
}

pub struct TwofaSettings {
    pub secret: Option<String>,
    pub window: Option<u32>,
    pub hash: Option<HashFunction>,
    pub encoding: Option<Encoding>
}

impl TwofaSettings {
    pub fn new() -> Self {
        Self {
            secret: None,
            window: None,
            hash: None,
            encoding: None,
        }
    }

    pub fn set_secret(&mut self, secret: String) -> &mut Self {
        self.secret = Some(secret);
        self
    }

    pub fn set_window(&mut self, window: Option<u32>) -> &mut Self {
        match window {
            Some(w) => {
                self.window = Some(w);
                self
            },
            None => {
                self.window = Some(30);
                self
            }
        }
    }

    pub fn set_hash(&mut self, hash: Option<HashFunction>) -> &mut Self {
        match hash {
            Some(hf) => {
                self.hash = Some(hf);
                self
            },
            None => {
                self.hash = Some(HashFunction::Sha512);
                self
            }
        }
    }

    pub fn set_encoding(&mut self, encoding: Option<Encoding>) -> &mut Self {
        match encoding {
            Some(en) => {
                self.encoding = Some(en);
                self
            },
            None => {
                self.encoding = Some(Encoding::Base32);
                self
            }
        }
    }

    pub fn to_json(&self) -> Value {
        let mut encoding = String::from("");
        let mut hash = String::from("");

        match &self.encoding {
            Some(e) => {
                match e {
                    Encoding::Hex => {
                        encoding.push_str("hex");
                    },
                    Encoding::Ascii => {
                        encoding.push_str("ascii");
                    },
                    Encoding::Base32 => {
                        encoding.push_str("base32");
                    },
                }
            },
            None => {
                encoding.push_str("base32");
            }
        };

        match &self.hash {
            Some(h) => {
                match h {
                    HashFunction::Sha256 => {
                        hash.push_str("sha256");
                    },
                    HashFunction::Sha512 => {
                        hash.push_str("sha512");
                    },
                    #[allow(deprecated)]
                    HashFunction::Sha1 => {
                        hash.push_str("sha1");
                    },
                };
            },
            None => {
                hash.push_str("sha512")
            }
        };

        serde_json::json!({
            "encoding": encoding,
            "window": self.window.clone().unwrap(),
            "hash": hash,
            "secret": self.secret.clone().unwrap(),
        })
    }
}

impl Display for TwofaSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hash_string = String::from("");
        let mut encoding = String::from("");
        match self.hash.unwrap() {
            HashFunction::Sha512 => {
                hash_string.push_str("SHA512");
            },
            HashFunction::Sha256 => {
                hash_string.push_str("SHA256");
            },
            #[allow(deprecated)]
            HashFunction::Sha1 => {
                hash_string.push_str("SHA1");
            }
        };

        match &self.encoding {
            Some(e) => {
                match e {
                    Encoding::Base32 => {
                        encoding.push_str("base32");
                    },
                    Encoding::Hex => {
                        encoding.push_str("hex");
                    },
                    Encoding::Ascii => {
                        encoding.push_str("ascii");
                    }
                }
            },
            None => {
                encoding.push_str("undefined");
            }
        }

        write!(f, "s: {}; h: {}; w: {}; e: {}",
                 self.secret.as_ref().unwrap(),
                    hash_string,
                self.window.as_ref().unwrap(),
                encoding,
                )
    }
}

pub fn create_twofa_settings_with_input(data: &Opts) -> Result<TwofaSettings, &'static str> {
    let mut settings: TwofaSettings = TwofaSettings::new();

    if let None = &data.secret {
        println!("Secret is needed. -s --secret");
        std::process::exit(0);
    }

    settings.set_secret(data.secret.clone().unwrap());

    let parsed_integer: u32 = data.window.as_u64().unwrap().try_into().unwrap();
    settings.set_window(Some(parsed_integer));

    match &data.hash[..] {
        "sha256" => {
            settings.set_hash(Some(HashFunction::Sha256));
        },
        "sha512" => {
            settings.set_hash(Some(HashFunction::Sha512));
        },
        #[allow(deprecated)]
        "sha1" => {
            settings.set_hash(Some(HashFunction::Sha1));
        },
        _ => {
            println!("Hash not supported");
            std::process::exit(0);
        }
    }

    match &data.encoding[..] {
        "base32" => {
            settings.set_encoding(Some(Encoding::Base32));
        },
        "ascii" => {
            settings.set_encoding(Some(Encoding::Ascii));
        },
        "hex" => {
            settings.set_encoding(Some(Encoding::Hex));
        },
        _ => {
            println!("Unsupported encoding");
            std::process::exit(1);
        }
    }

    Ok(settings)
}

pub fn create_twofa_settings(data: Option<Map<String, Value>>) -> Result<TwofaSettings, &'static str>{
    let mut settings: TwofaSettings = TwofaSettings::new();

    match data {
        Some(map) => {
            for item in map.iter() {
                match &item.0[..] {
                    "secret" => {
                        if let Value::String(secret) = item.1.clone() {
                            settings.set_secret(secret.clone());
                        } else {
                            return Err("Mismatched secret stored");
                        }
                    },
                    "window" => {
                        if let Value::Number(window) = item.1.clone() {
                            let parsed_integer = window.as_u64().expect("Could not parse integer 'window'");
                            let parsed_to_u32: u32 = parsed_integer.try_into().expect("Could not parse 'window' to u32");
                            settings.set_window(Some(parsed_to_u32));
                        } else {
                            return Err("Mismatched window type stored");
                        }
                    },
                    "hash" => {
                        if let Value::String(hash) = item.1.clone() {
                            match hash.as_str() {
                                "sha512" => {
                                    settings.set_hash(Some(HashFunction::Sha512));
                                },
                                "sha256" => {
                                    settings.set_hash(Some(HashFunction::Sha256));
                                },
                                #[allow(deprecated)]
                                "sha1" => {
                                    settings.set_hash(Some(HashFunction::Sha1));
                                },
                                _ => {
                                    return Err("Mismatched hash stored");
                                }
                            };
                        }
                    },
                    "encoding" => {
                        if let Value::String(encoding) = item.1.clone() {
                            match encoding.as_str() {
                                "base32" => {
                                    settings.set_encoding(Some(Encoding::Base32));
                                },
                                "hex" => {
                                    settings.set_encoding(Some(Encoding::Hex));
                                },
                                "ascii" => {
                                    settings.set_encoding(Some(Encoding::Ascii));
                                },
                                _ => {
                                    return Err("Unknown encoding saved");
                                }
                            }
                        }
                    },
                    _ => {
                        println!("Value not needed: {}", &item.0[..]);
                    }
                }
            };
            Ok(settings)
        },
        None => {
            Err("No data for application saved")
        }
    }
}

pub fn create_code_with_twofa_settings(ts: &TwofaSettings) -> Result<String, &'static str> {
    let mut builder = TOTPBuilder::new();

    match &ts.secret {
        Some(secret) => {
            if let Some(encoding) = &ts.encoding {
                match encoding {
                    Encoding::Ascii => {
                        builder.ascii_key(secret.as_str());
                    },
                    Encoding::Hex => {
                        builder.hex_key(secret.as_str());
                    },
                    Encoding::Base32 => {
                        builder.base32_key(secret.as_str());
                    }
                };
            } else {
                return Err("Encoding is missing");
            }
        },
        None => {
            return Err("Secret is missing");
        }
    };

    if let Some(window) = ts.window {
        builder.period(window);
    } else {
        builder.period(30);
    }

    if let Some(hash) = ts.hash {
        builder.hash_function(hash);
    } else {
        builder.hash_function(HashFunction::Sha512);
    }

    let code = builder.finalize().expect("Could not generate code").generate();

    Ok(code)
}
