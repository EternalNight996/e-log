$ErrorActionPreference = 'Stop'

$combos = @(
  @{ Name = 'default (log)';           Args = @('--lib', '--examples') },
  @{ Name = 'log + tracing';           Args = @('--features', 'tracing', '--lib', '--examples') },
  @{ Name = 'tracing full';            Args = @('--no-default-features', '--features', 'tracing,panic,dialog,tracing-appender,tracing-subscriber', '--lib', '--examples') },
  @{ Name = 'log + panic + dialog';    Args = @('--features', 'panic,dialog', '--lib', '--examples') },
  @{ Name = 'no features';             Args = @('--no-default-features', '--lib', '--examples') }
)

foreach ($c in $combos) {
  Write-Host "== $($c.Name) =="
  cargo check @($c.Args)
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
Write-Host 'all feature combos OK'
