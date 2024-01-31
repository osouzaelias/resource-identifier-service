use super::*;
use crate::Input;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_all_fields_present() {
        let input = Input {
            legal_entity: "Corporation".to_string(),
            tenant: "Banking".to_string(),
            segment: "Retail".to_string(),
            payment_instrument: "CreditCard".to_string(),
            customer_id: "123456789".to_string(),
        };
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_validate_input_missing_field() {
        let input = Input {
            legal_entity: "".to_string(),
            tenant: "Banking".to_string(),
            segment: "Retail".to_string(),
            payment_instrument: "CreditCard".to_string(),
            customer_id: "123456789".to_string(),
        };
        assert!(validate_input(&input).is_err());
    }

    #[test]
    fn test_ris_format() {
        let input = Input {
            legal_entity: "Itau".to_string(),
            tenant: "Banking".to_string(),
            segment: "Retail".to_string(),
            payment_instrument: "CreditCard".to_string(),
            customer_id: "123456789".to_string(),
        };

        let ris = create_ris(&input);
        let re = Regex::new(r"^ris:[\w]+:[\w]+:[\w]+:[\w]+:[\w]+$").unwrap();
        assert!(re.is_match(&ris));
    }
}