extern crate android_sdk_cli;
extern crate hyper;

use android_sdk_cli::xmlhelper;
use android_sdk_cli::repository11;

fn main() {
    let client = hyper::Client::new();
    let response = client.get(repository11::XML_URL).send().unwrap();
    let stream = xmlhelper::parse(response).unwrap();
    let sdk_repository = repository11::parse_sdk_repository(&mut stream.into_iter()).unwrap();

    for ndk in sdk_repository.ndks {
        println!("{:?}", ndk);
    }
    for platform in sdk_repository.platforms {
        println!("{:?}", platform);
    }
    for source in sdk_repository.sources {
        println!("{:?}", source);
    }
    for build_tool in sdk_repository.build_tools {
        println!("{:?}", build_tool);
    }
    for platform_tool in sdk_repository.platform_tools {
        println!("{:?}", platform_tool);
    }
}
