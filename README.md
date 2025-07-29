# rnd
A CLI tool to get random number

## Usahe

Get random number:

```bash
$ rnd
1226938335566561059
```

Get random number for range:

```bash
$ rnd -10..=10
7
```

Get random number for count:

```bash
$ rnd -10..10 -n 5
-1
0
0
-3
2
```

Get random float number:

```bash
$ rnd -f
0.09189088878360863
```

## Why?

It would be useful in some scenarios, for example:

```bash
while true;
do 
curl 'https://api.live.bilibili.com/msg/send' \
    --data-urlencode 'roomid=1961605007' \
    --data-urlencode 'msg=prpr' \
    --data-urlencode 'csrf=xxx' \
    -b 'SESSDATA=xxx;bili_jct=xxx' \
    --data-urlencode 'fontsize=25' \
    --data-urlencode 'color=16777215' \
    --data-urlencode "rnd=$(date +%s)";
    sleep $(rnd 5..=10);
done
```
