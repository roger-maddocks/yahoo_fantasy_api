use crate::factories;
use crate::models::fantasy_week::FantasyWeek;
use crate::models::report::Report;

pub struct ReportCache<T> where T: Fn(&FantasyWeek) -> Option<Report>
{
    report_request: T,
    report: Option<Report>
}

impl<T> ReportCache<T> where T: Fn(&FantasyWeek) -> Option<Report>
{
    pub fn new(report_request: T) -> ReportCache<T> {
        ReportCache {
            report_request,
            report: None,
        }
    }

    pub async fn get_report(&mut self, week: u64) -> Option<Report> {
        match self.report.clone() {
            Some(r) => Some(r),
            None => {
                let r = (self.report_request)(&FantasyWeek::new(week, week));// (&FantasyWeek(week, week));
                self.report = r.clone();
                r.clone()
            }
        }
    }
}

// let fantasy_week = &FantasyWeek::new(week, week);
