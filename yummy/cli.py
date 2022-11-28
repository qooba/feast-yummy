import click
import yummy_rs
import yummy_mlflow


@click.group()
@click.pass_context
def cli(
    ctx: click.Context
):
    pass

@cli.command("serve")
@click.option("--host","-h",
              type=click.STRING,
              default="127.0.0.1",
              show_default=True,
              help='Host for the feature server.'
              )
@click.option("--port","-p",
              type=click.INT,
              default=6566,
              show_default=True,
              help='Port for the feature server.'
              )
@click.option("--filename","-f",
              type=click.STRING,
              default="feature_store.yaml",
              show_default=True,
              help='Configuration filename for the feature server.'
              )
@click.option("--log-level",
              type=click.STRING,
              default="error",
              show_default=True,
              help='Log level for the feature server. One of: debug, info, error, warn'
              )
@click.pass_context
def serve_command(
    ctx: click.Context,
    host: str,
    port: int,
    filename: str,
    log_level: str,
):
    """Start rust feature server."""
    yummy_rs.serve(filename, host, port, log_level)

@cli.group(name='models')
def models_cmd():
    """Models related commands"""
    pass

@models_cmd.command("serve")
@click.option("--host","-h",
              type=click.STRING,
              default="127.0.0.1",
              show_default=True,
              help='Host for the feature server.'
              )
@click.option("--port","-p",
              type=click.INT,
              default=6566,
              show_default=True,
              help='Port for the feature server.'
              )
@click.option("--model-uri","-m",
              type=click.STRING,
              required=True,
              help='Local model path.'
              )
@click.option("--log-level",
              type=click.STRING,
              default="error",
              show_default=True,
              help='Log level for the feature server. One of: debug, info, error, warn'
              )
@click.pass_context
def serve_command(
    ctx: click.Context,
    host: str,
    port: int,
    model_uri: str,
    log_level: str,
):
    """Start rust feature server."""
    yummy_mlflow.model_serve(model_uri, host, port, log_level)


if __name__ == '__main__':
    cli()
