use whatlang::{Detector, Lang};

pub struct Matcher {
    detector: Detector,
    target: Lang,
    threshold: f64,
    invert_match: bool,
}

impl Default for Matcher {
    fn default() -> Matcher {
        Matcher {
            detector: Detector::new(),
            target: Lang::Eng,
            threshold: 0.0,
            invert_match: false,
        }
    }
}

impl Matcher {
    pub fn new(pool: Vec<Lang>, target: Lang, threshold: f64, invert_match: bool) -> Self {
        let detector: Detector = if pool.len() > 1 {
            Detector::with_allowlist(pool)
        } else {
            Detector::new()
        };

        Self {
            detector,
            target,
            threshold,
            invert_match,
        }
    }

    pub fn is_lang(&self, line: &str) -> bool {
        if let Some(d) = self.detector.detect(line) {
            if d.lang() == self.target && d.confidence() >= self.threshold {
                return true ^ self.invert_match;
            }
        }
        false ^ self.invert_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        let text = "Además de todo lo anteriormente dicho, también encontramos...";
        let matcher = Matcher::new(vec![Lang::Spa, Lang::Eng, Lang::Bul], Lang::Spa, 1.0, false);
        assert_eq!(matcher.is_lang(text), true);
    }

    #[test]
    fn test_inverted_match() {
        let text = "Además de todo lo anteriormente dicho, también encontramos...";
        let matcher = Matcher::new(vec![Lang::Spa, Lang::Eng, Lang::Bul], Lang::Spa, 1.0, true);
        assert_eq!(matcher.is_lang(text), false);
    }
}
