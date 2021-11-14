use celcat::{
    fetchable::calendar::{CalView, CalendarData, CalendarDataRequest},
    Celcat, Group, GroupId,
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let mut celcat = Celcat::new("https://services-web.u-cergy.fr/calendar").await?;
    celcat.login(&opts.username, &opts.password).await?;

    match opts.subcmd {
        SubCommand::Calendar { start, end, id } => {
            let res: CalendarData<Group> = celcat
                .fetch(CalendarDataRequest {
                    start,
                    end,
                    res_type: Group,
                    cal_view: CalView::Month,
                    federation_ids: GroupId(id),
                    colour_scheme: 3,
                })
                .await?;
            println!("{:#?}", res);
        }
    }

    Ok(())
}
