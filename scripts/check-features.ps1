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

# tag 日志回归：`emit_tag` 必须在 log 与 tracing 两条路径都可用。
# 背景：log 路径下短名宏是 log crate 的再导出，不接受 `tag = …`；
# 走函数式入口 `e_log::tag::emit_tag` 才能两条路径通吃。
$tagCombos = @(
  @{ Name = 'tag (log)';         Feat = 'log,_tag_any' },
  @{ Name = 'tag (tracing)';     Feat = 'tracing,_tag_any' },
  @{ Name = 'tag (log,tracing)'; Feat = 'log,tracing,_tag_any' },
  @{ Name = 'tag (log,_tag)';    Feat = 'log,_tag' }
)

foreach ($c in $tagCombos) {
  Write-Host "== $($c.Name) =="
  cargo run --quiet --example tag_fn --no-default-features --features $c.Feat
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

Write-Host 'all feature combos OK'
