use celcat::{
    fetchable::{
        calendar::{CalView, CalendarData, CalendarDataRequest},
        event::{Event, EventRequest},
    },
    Celcat, CourseId, Student, StudentId,
};
use chrono::NaiveDateTime;
use clap::Parser;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long)]
    username: String,
    #[clap(short, long)]
    password: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    Calendar {
        #[clap(short, long)]
        start: NaiveDateTime,
        #[clap(short, long)]
        end: NaiveDateTime,
        #[clap(short, long)]
        id: String,
    },
    Event {
        #[clap(short, long)]
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let opts: Opts = Opts::parse();

    let mut celcat = Celcat::new("https://services-web.u-cergy.fr/calendar").await?;
    celcat.login(&opts.username, &opts.password).await?;

    match opts.subcmd {
        SubCommand::Calendar { start, end, id } => {
            let res: CalendarData<Student> = celcat
                .fetch(CalendarDataRequest {
                    start,
                    end,
                    res_type: Student,
                    cal_view: CalView::Month,
                    federation_ids: StudentId(id),
                    colour_scheme: 3,
                })
                .await?;
            println!("{:#?}", res);
        }
        SubCommand::Event { id } => {
            let res: Event = celcat
                .fetch(EventRequest {
                    event_id: CourseId(id),
                })
                .await?;
            println!("{:#?}", res);
        }
    }

    Ok(())
}
