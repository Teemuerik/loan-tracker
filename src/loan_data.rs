// In this program, all loans are calculated with the basic Finnish system.
// All months have 30 days, and the year has exactly 360 days in this system.

const YEAR_DAYS: i32 = 360;
const MONTH_DAYS: i32 = 30;

#[derive(Copy, Clone)]
struct LoanDate {
    day: u8,
    month: u8,
    year: i32,
}

impl LoanDate {
    fn days_to(&self, target: LoanDate) -> i32 {
        let year_count: i32 = target.year - self.year;
        let month_count: i8 = target.month as i8 - self.month as i8;
        let day_count: i8 = target.day as i8 - self.day as i8;

        day_count as i32 + month_count as i32 * MONTH_DAYS + year_count * YEAR_DAYS
    }
}

struct LoanState {
    principal_cents: u64,
}

struct LoanContract {}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that the day_difference function gives the
    // expected difference between two dates.
    fn test_date_diff(
        first_day: u8,
        first_month: u8,
        first_year: i32,
        second_day: u8,
        second_month: u8,
        second_year: i32,
        expected_diff: i32,
    ) {
        let first_date = LoanDate {
            day: first_day,
            month: first_month,
            year: first_year,
        };
        let second_date = LoanDate {
            day: second_day,
            month: second_month,
            year: second_year,
        };

        assert_eq!(first_date.days_to(second_date), expected_diff);
    }

    #[test]
    fn test_days_to_calc() {
        // Run a few tests on the date diff calculations.
        test_date_diff(1, 1, 2026, 1, 2, 2026, 30);
        test_date_diff(1, 1, 2026, 1, 1, 2027, 360);
        test_date_diff(15, 1, 2026, 15, 2, 2026, 30);
        test_date_diff(30, 1, 2026, 1, 2, 2026, 1);
        test_date_diff(30, 12, 2026, 1, 1, 2027, 1);
        test_date_diff(1, 9, 2026, 1, 3, 2027, 180);
    }
}
