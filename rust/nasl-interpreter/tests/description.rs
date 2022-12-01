#[cfg(test)]
mod tests {

    use nasl_interpreter::{interpret, Mode};
    use nasl_interpreter::NaslValue;
    
    use sink::DefaultSink;
    use sink::NVTField::*;
    use sink::NvtRef;
    use sink::StoreType::NVT;
    use sink::Sink;
    use sink::TagKey::*;
    use sink::ACT::*;

    #[test]
    fn description() {
        let code = r###"
if(description)
{
  script_oid("0.0.0.0.0.0.0.0.0.1");
  script_version("2022-11-14T13:47:12+0000");
  script_tag(name:"creation_date", value:"2013-04-16 11:21:21 +0530 (Tue, 16 Apr 2013)");
  script_name("that is a very long and descriptive name");
  script_category(ACT_DENIAL);
  script_copyright("Copyright (C) 2022 Greenbone Networks GmbH");
  script_family("Denial of Service");
  script_dependencies("ssh_detect.nasl", "ssh2.nasl");
  script_require_ports("Services/ssh", 22);
  script_mandatory_keys("ssh/blubb/detected");
  script_xref(name:"URL", value:"http://freshmeat.sourceforge.net/projects/eventh/");
  script_exclude_keys("Settings/disable_cgi_scanning", "bla/bla");
  #script_add_preference(name:"Enable Password", type:"password", value:"", id:2);
  script_require_udp_ports("Services/udp/unknown", 17);
  script_cve_id("CVE-1999-0524");
  script_require_keys("WMI/Apache/RootPath");
  exit(0);
}
        "###;
        let storage = DefaultSink::new(true);
        let results = interpret(&storage, Mode::Description("test.nasl"), code);
        assert_eq!(results, vec![Ok(NaslValue::Exit(0))]);
        assert_eq!(
            &storage.get("test.nasl", sink::GetType::NVT(None)).unwrap(),
            &vec![
                NVT(FileName("test.nasl".to_owned())),
                NVT(Oid("0.0.0.0.0.0.0.0.0.1".to_owned())),
                NVT(NoOp),
                NVT(Tag(
                    CreationDate,
                    "2013-04-16 11:21:21 +0530 (Tue, 16 Apr 2013)".to_owned()
                )),
                NVT(Name("that is a very long and descriptive name".to_owned())),
                NVT(Category(Denial)),
                NVT(NoOp),
                NVT(Family("Denial of Service".to_owned())),
                NVT(Dependencies(vec![
                    "ssh_detect.nasl".to_owned(),
                    "ssh2.nasl".to_owned()
                ])),
                NVT(RequiredPorts(vec![
                    "Services/ssh".to_owned(),
                    "22".to_owned()
                ])),
                NVT(MandatoryKeys(vec!["ssh/blubb/detected".to_owned()])),
                NVT(Reference(NvtRef {
                    class: "http://freshmeat.sourceforge.net/projects/eventh/".to_owned(),
                    id: "URL".to_owned(),
                    text: None
                })),
                NVT(ExcludedKeys(vec![
                    "Settings/disable_cgi_scanning".to_owned(),
                    "bla/bla".to_owned()
                ])),
                NVT(RequiredUdpPorts(vec![
                    "Services/udp/unknown".to_owned(),
                    "17".to_owned()
                ])),
                NVT(Reference(NvtRef {
                    class: "cve".to_owned(),
                    id: "CVE-1999-0524".to_owned(),
                    text: None
                })),
                NVT(RequiredKeys(vec!["WMI/Apache/RootPath".to_owned()]))
            ]
        );
    }
}
