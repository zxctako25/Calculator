pub struct NumberFormatter;

impl NumberFormatter {
    pub fn show(v: f64) -> String {
        Self::group(&Self::plain(v))
    }

    pub fn plain(v: f64) -> String {
        if v == 0.0 {
            return "0".to_string();
        }

        let a = v.abs();
        if a >= 1e16 || a < 1e-9 {
            let s = format!("{:.14e}", v);
            let (m, e) = s.split_once('e').unwrap();
            let m = if m.contains('.') {
                m.trim_end_matches('0').trim_end_matches('.')
            } else {
                m
            };

            let sign = if e.starts_with('-') { "" } else { "+" };
            return format!("{}e{}{}", m, sign, e);
        }

        let digits_before = a.log10().floor() as i32 + 1;
        let decimals = (16 - digits_before).clamp(0, 24) as usize;
        let s = format!("{:.*}", decimals, v);
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s
        }
    }

    pub fn group(raw: &str) -> String {
        if raw.contains('e') {
            return raw.replace('.', ",");
        }

        let (neg, body) = match raw.strip_prefix('-') {
            Some(b) => (true, b),
            None => (false, raw),
        };

        let (int, frac) = match body.split_once('.') {
            Some((i, f)) => (i, Some(f)),
            None => (body, None),
        };

        let mut out = String::new();
        if neg {
            out.push('-');
        }

        let len = int.chars().count();
        for (i, ch) in int.chars().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                out.push(' ');
            }
            out.push(ch);
        }

        if let Some(f) = frac {
            out.push(',');
            out.push_str(f);
        }
        out
    }
}
