use std::collections::HashMap;
use xmlhelper::Event;

pub static XML_URL_BASE: &'static str = "https://dl.google.com/android/repository";
pub static XML_URL: &'static str = "https://dl.google.com/android/repository/repository-11.xml";

#[derive(Debug)]
pub struct SdkRepository {
    pub licenses: HashMap<String, String>,
    pub ndks: Vec<Ndk>,
    pub platforms: Vec<Platform>,
    pub sources: Vec<Source>,
    pub build_tools: Vec<BuildTool>,
    pub platform_tools: Vec<PlatformTool>,
}

#[derive(Debug)]
pub struct Ndk {
    pub revision: u32,
    pub uses_license: Option<String>,
    pub archives: Vec<Archive>,
}

#[derive(Debug)]
pub struct Platform {
    pub api_level: u32,
    pub revision: u32,
    pub archives: Vec<Archive>,
    pub uses_license: Option<String>,
}

#[derive(Debug)]
pub struct Archive {
    pub checksum: String,
    pub url: String,
    pub host_os: Option<OsType>,
    pub host_bits: Option<BitSize>,
}

impl Archive {
    pub fn absolute_url(&self) -> String {
        return format!("{}/{}", XML_URL_BASE, self.url);
    }
}

#[derive(Debug)]
pub enum OsType { Linux, Macosx, Windows }

#[derive(Debug)]
pub enum BitSize { Bit32, Bit64 }

#[derive(Debug)]
pub struct Source {
    pub api_level: u32,
    pub revision: u32,
    pub archives: Vec<Archive>,
    pub uses_license: Option<String>,
}

#[derive(Debug)]
pub struct BuildTool {
    pub revision: Revision,
    pub uses_license: Option<String>,
    pub archives: Vec<Archive>,
}

#[derive(Debug)]
pub struct Revision {
    pub major: u32,
    pub minor: Option<u32>,
    pub micro: Option<u32>,
    pub preview: Option<u32>,
}

#[derive(Debug)]
pub struct PlatformTool {
    pub revision: Revision,
    pub uses_license: Option<String>,
    pub archives: Vec<Archive>,
}

pub fn parse_sdk_repository<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<SdkRepository, String> {
    let mut sdk_repository = SdkRepository {
        licenses: HashMap::new(),
        ndks: Vec::new(),
        platforms: Vec::new(),
        sources: Vec::new(),
        build_tools: Vec::new(),
        platform_tools: Vec::new(),
    };
    match stream.next() {
        Some(Event::StartElement { ref local_name, .. }) if local_name == "sdk-repository" => {
            // ok
        }
        _ => {
            return Err("not sdk-repository".to_string());
        }
    }
    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, attributes }) => {
                if local_name == "license" {
                    let license = try!(parse_string(&mut stream, local_name));
                    if let Some(id) = attributes.get("id") {
                        sdk_repository.licenses.insert(id.clone(), license);
                    } else {
                        return Err("license element does not have id attribute".to_string());
                    }
                } else if local_name == "ndk" {
                    sdk_repository.ndks.push(try!(parse_ndk(&mut stream)));
                } else if local_name == "platform" {
                    sdk_repository.platforms.push(try!(parse_platform(&mut stream)));
                } else if local_name == "source" {
                    sdk_repository.sources.push(try!(parse_source(&mut stream)));
                } else if local_name == "build-tool" {
                    sdk_repository.build_tools.push(try!(parse_build_tool(&mut stream)));
                } else if local_name == "platform-tool" {
                    sdk_repository.platform_tools.push(try!(parse_platform_tool(&mut stream)));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "sdk-repository" {
                    return Ok(sdk_repository);
                }
            }
            Some(_) => {}
            None => { return Err(format!("parse error during sdk-repository")); }
        }
    }
}

fn parse_string<I: Iterator<Item=Event>>(mut stream: &mut I, name: String) -> Result<String, String> {
    let mut result = String::new();

    loop {
        match stream.next() {
            Some(Event::Text { text }) => {
                result = text;
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == name {
                    return Ok(result);
                } else {
                    return Err(format!("unexpected end element while parsing {}: {}", name, local_name));
                }
            }
            e => { return Err(format!("parse error during license: {:?}", e)); }
        }
    }
}

fn parse_ndk<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<Ndk, String> {
    let mut ndk = Ndk {
        revision: 0,
        uses_license: None,
        archives: Vec::new(),
    };

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, attributes }) => {
                if local_name == "revision" {
                    ndk.revision = try!(parse_u32(&mut stream, local_name));
                } else if local_name == "uses-license" {
                    ndk.uses_license = Some(try!(convert_uses_license(attributes)));
                } else if local_name == "archives" {
                    ndk.archives = try!(parse_archives(&mut stream));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "ndk" {
                    return Ok(ndk);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during ndk".to_string()); }
        }
    }
}

fn convert_uses_license(attributes: HashMap<String, String>) -> Result<String, String> {
    if let Some(r) = attributes.get("ref") {
        return Ok(r.clone());
    } else {
        return Err("uses-license element does not have ref attribute".to_string());
    }
}

fn parse_platform<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<Platform, String> {
    let mut platform = Platform {
        archives: Vec::new(),
        uses_license: None,
        api_level: 0,
        revision: 0,
    };

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, attributes }) => {
                if local_name == "uses-license" {
                    if let Some(r) = attributes.get("ref") {
                        platform.uses_license = Some(r.clone());
                    } else {
                        return Err("uses-license element does not have ref attribute".to_string());
                    }
                } else if local_name == "api-level" {
                    platform.api_level = try!(parse_u32(&mut stream, local_name));
                } else if local_name == "revision" {
                    platform.revision = try!(parse_u32(&mut stream, local_name));
                } else if local_name == "archives" {
                    platform.archives = try!(parse_archives(&mut stream));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "platform" {
                    return Ok(platform);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during platform".to_string()); }
        }
    }
}

fn parse_u32<I: Iterator<Item=Event>>(mut stream: &mut I, name: String) -> Result<u32, String> {
    let mut result = 0;

    loop {
        match stream.next() {
            Some(Event::Text { text }) => {
                if let Ok(i) = text.parse::<u32>() {
                    result = i;
                } else {
                    return Err(format!("non-positive-integer {} was found: {}", name, text));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == name {
                    return Ok(result);
                } else {
                    return Err(format!("unexpected end element while parsing {}: {}", name, local_name));
                }
            }
            e => { return Err(format!("parse error during {}: {:?}", name, e)); }
        }
    }
}

fn parse_archives<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<Vec<Archive>, String> {
    let mut result = Vec::new();

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, .. }) => {
                if local_name == "archive" {
                    result.push(try!(parse_archive(&mut stream)));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "archives" {
                    return Ok(result);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during archives".to_string()); }
        }
    }
}

fn parse_archive<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<Archive, String> {
    let mut result = Archive {
        checksum: String::new(),
        url: String::new(),
        host_os: None,
        host_bits: None,
    };

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, .. }) => {
                if local_name == "checksum" {
                    result.checksum = try!(parse_string(&mut stream, local_name));
                } else if local_name == "url" {
                    result.url = try!(parse_string(&mut stream, local_name));
                } else if local_name == "host-os" {
                    result.host_os = Some(try!(parse_host_os(&mut stream)));
                } else if local_name == "host-bits" {
                    result.host_bits = Some(try!(parse_host_bits(&mut stream)));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "archive" {
                    return Ok(result);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during archive".to_string()); }
        }
    }
}

fn parse_host_os<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<OsType, String> {
    let mut result = OsType::Linux;

    loop {
        match stream.next() {
            Some(Event::Text { text }) => {
                if text == "linux" {
                    result = OsType::Linux;
                } else if text == "macosx" {
                    result = OsType::Macosx;
                } else if text == "windows" {
                    result = OsType::Windows;
                } else {
                    return Err(format!("unknown host-os value: {}", text));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "host-os" {
                    return Ok(result);
                } else {
                    return Err(format!("unexpected end element while parsing {}: {}", "host-os", local_name));
                }
            }
            e => { return Err(format!("parse error during {}: {:?}", "host-os", e)); }
        }
    }
}

fn parse_host_bits<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<BitSize, String> {
    let mut result = BitSize::Bit32;

    loop {
        match stream.next() {
            Some(Event::Text { text }) => {
                if text == "32" {
                    result = BitSize::Bit32;
                } else if text == "64" {
                    result = BitSize::Bit64;
                } else {
                    return Err(format!("unknown {} value: {}", "host-bits", text));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "host-bits" {
                    return Ok(result);
                } else {
                    return Err(format!("unexpected end element while parsing {}: {}", "host-bits", local_name));
                }
            }
            e => { return Err(format!("parse error during {}: {:?}", "host-bits", e)); }
        }
    }
}

fn parse_source<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<Source, String> {
    let mut source = Source {
        archives: Vec::new(),
        uses_license: None,
        api_level: 0,
        revision: 0,
    };

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, attributes }) => {
                if local_name == "uses-license" {
                    if let Some(r) = attributes.get("ref") {
                        source.uses_license = Some(r.clone());
                    } else {
                        return Err("uses-license element does not have ref attribute".to_string());
                    }
                } else if local_name == "api-level" {
                    source.api_level = try!(parse_u32(&mut stream, local_name));
                } else if local_name == "revision" {
                    source.revision = try!(parse_u32(&mut stream, local_name));
                } else if local_name == "archives" {
                    source.archives = try!(parse_archives(&mut stream));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "source" {
                    return Ok(source);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during source".to_string()); }
        }
    }
}

fn parse_build_tool<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<BuildTool, String> {
    let mut build_tool = BuildTool {
        revision: Revision {
            major: 0,
            minor: None,
            micro: None,
            preview: None,
        },
        archives: Vec::new(),
        uses_license: None,
    };

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, attributes }) => {
                if local_name == "revision" {
                    build_tool.revision = try!(parse_revision(&mut stream));
                } else if local_name == "uses-license" {
                    build_tool.uses_license = Some(try!(convert_uses_license(attributes)));
                } else if local_name == "archives" {
                    build_tool.archives = try!(parse_archives(&mut stream));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "build-tool" {
                    return Ok(build_tool);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during build-tool".to_string()); }
        }
    }
}

fn parse_revision<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<Revision, String> {
    let mut revision = Revision {
        major: 0,
        minor: None,
        micro: None,
        preview: None,
    };

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, .. }) => {
                if local_name == "major" {
                    revision.major = try!(parse_u32(&mut stream, local_name));
                } else if local_name == "minor" {
                    revision.minor = Some(try!(parse_u32(&mut stream, local_name)));
                } else if local_name == "micro" {
                    revision.micro = Some(try!(parse_u32(&mut stream, local_name)));
                } else if local_name == "preview" {
                    revision.preview = Some(try!(parse_u32(&mut stream, local_name)));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "revision" {
                    return Ok(revision);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during revision".to_string()); }
        }
    }
}

fn parse_platform_tool<I: Iterator<Item=Event>>(mut stream: &mut I) -> Result<PlatformTool, String> {
    let mut platform_tool = PlatformTool {
        revision: Revision {
            major: 0,
            minor: None,
            micro: None,
            preview: None,
        },
        archives: Vec::new(),
        uses_license: None,
    };

    loop {
        match stream.next() {
            Some(Event::StartElement { local_name, attributes }) => {
                if local_name == "revision" {
                    platform_tool.revision = try!(parse_revision(&mut stream));
                } else if local_name == "uses-license" {
                    platform_tool.uses_license = Some(try!(convert_uses_license(attributes)));
                } else if local_name == "archives" {
                    platform_tool.archives = try!(parse_archives(&mut stream));
                }
            }
            Some(Event::EndElement { local_name }) => {
                if local_name == "platform-tool" {
                    return Ok(platform_tool);
                }
            }
            Some(_) => {}
            None => { return Err("parse error during platform-tool".to_string()); }
        }
    }
}
