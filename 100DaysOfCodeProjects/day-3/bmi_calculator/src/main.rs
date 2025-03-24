fn calculate_bmi(weight: f32, height: f32) -> f32 {
    let bmi: f32 = weight / (height * height);
    return bmi;
}


fn output_bmi(bmi: f32) -> String {
    if bmi < 18.5 {
        let underweight = String::from("Underweight");
        underweight
    } else if bmi >= 18.5 && bmi < 25.0 {
        let normal_weight = String::from("Normal Weight");
        normal_weight
    } else {
        let overweight = String::from("Overweight");
        overweight
    }
}


fn main() {
    let weight = 85.0;
    let height = 1.85;


    let calculation = calculate_bmi(weight, height);


    let bmi = output_bmi(calculation);


    println!("Weight is {}; height is {}; calculation is {:.2}; BMI is: {}.", weight, height, calculation, bmi);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_calculate_bmi() {
        assert_eq!(calculate_bmi(400.0, 10.0), 4.0);
    }


    #[test]
    fn test_output_bmi() {
        assert_eq!(output_bmi(18.0), "Underweight");
        assert_eq!(output_bmi(19.0), "Normal Weight");
        assert_eq!(output_bmi(27.0), "Overweight");
    }
}


