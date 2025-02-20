use super::*;

/// Maps a json file into a linear assigned multilevel key/value array
///
/// Viperus will make use of the adaptor, by opting in the feature `"fmt-json"`,
/// which will consume the `serde_json` crate internally.
pub struct JsonAdapter {
    source: String,
    data: serde_json::Map<String, serde_json::Value>,
    //config_map: crate::map::Map,
}

impl JsonAdapter {
    pub fn new() -> Self {
        JsonAdapter {
            source: String::default(),
            data: serde_json::Map::new(),
            //config_map: crate::map::Map::new(),
        }
    }

    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
        let mut f = std::fs::File::open(name)?;
        self.load(&mut f)
    }

    pub fn load<R: std::io::Read>(&mut self, source: &mut R) -> AdapterResult<()> {
        self.source.truncate(0);

        source.read_to_string(&mut self.source)?;
        Ok(())
    }
}
impl ConfigAdapter for JsonAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data =
            serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&self.source)?;

        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

        //let mut kpath;

        for (k, v) in self.data.iter() {
            let kpath = k.to_owned();

            rec_json(&mut res, &kpath, v);
        }

        res
    }
}

fn rec_json(config_map: &mut crate::map::Map, kpath: &str, v: &serde_json::Value) {
    debug!("{:?} => {:?}", kpath, v);

    match v {
        serde_json::Value::Object(m) => {
            for (kk, vv) in m {
                let kk = format!("{}.{}", kpath, kk);
                rec_json(config_map, &kk, vv);
            }
        }

        serde_json::Value::String(s) => {
            config_map.add(kpath, s.clone());
        }

        serde_json::Value::Bool(b) => {
            config_map.add(kpath, *b);
        }

        _ => (),
    }
}
