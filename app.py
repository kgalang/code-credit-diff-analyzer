import code_credit_diff

all_stats = code_credit_diff.analyze("./fixtures/rustball.diff")

for stats in all_stats:
    print(stats.cleaned_removed)