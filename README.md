# Overview

A rustlet is a Rust closure that is used to extend the capabilities of servers that host applications accessed by means of the HTTP request-response programming model.

The rustlet [macro library](https://37miners.github.io/rustlet/librustlet/index.html) provides interfaces for writing rustlets. Rustlets are implemented via the [rustlet! macro](https://37miners.github.io/rustlet/librustlet/macro.rustlet.html). The other macros in the library provide all the functionalities required to respond to an HTTP Get or Post request.

# Execution of Rustlets

Rustlets are executed in the [nioruntime](https://github.com/37miners/nioruntime). This allows for performant execution using epoll on linux, kqueues on bsd variants including macos, and wepoll on windows.

# RSPs

RSPs are rust server pages. A RSP page is a text document that contains two types of text: static data, which can be expressed in any text-based format (but most commonly HTML), and rustlet tags which execute a specified rustlet as part of the page that is loading. A sample RSP may look like this:

```
<html>
    <head>
        <title>Sample RSP</title>
    </head>
    <body>
        <@=header>
        static content
        <@=middlecontent>
        static content
        <@=footer>
    </body>
</html>
```

In this example '<@=header>', '<@=middlecontent>', and '<@=footer>' are each rustlets that share the same parameters as the RSP page when executed. RSPs can be placed anywhere in the HTTP server's webroot and the rustlet container will interpret them to their dynamic form. RSP files must end with the .rsp extension so that the rustlet container knows to execute them as RSPs.

Please note that RSPs do not currently support async rustlets. If you embed a rustlet that uses the async_context or async_complete macros, it will result in undefined behaviour. Support for this is on the list of TODOs.

# Logging

The rustlet container comes with a logging library. The full documentation of the logging library can be [found here](https://37miners.github.io/rustlet/nioruntime_log/). This logging library uses the same syntax of the standard logging library for rust. See the example for info [here](https://37miners.github.io/rustlet/nioruntime_log/macro.info.html). Log level is set per file as seen in the previous example. The rustlet container itself uses this logging library for three log files. Each log file has a configurable location, max_size, and max_age. Further details about each of these log files is below.

## Main log

The mainlog prints out general messages. It mostly deals with log rotation messages and also prints out unexpected errors and information that that user should know about. On startup, all parameters are logged to the mainlog which is initially configured to print to standard out as well as it's file location. After startup, the mainlog is switched to only log to its file. A sample output of the mainlog looks like this:

```
[2021-09-07 20:10:09]: Rustlet Httpd 0.0.1
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-07 20:10:09]: root_dir:             '/Users/Shared/.niohttpd'
[2021-09-07 20:10:09]: webroot:              '/Users/Shared/.niohttpd/www'
[2021-09-07 20:10:09]: bind_address:         0.0.0.0:8080
[2021-09-07 20:10:09]: thread_pool_size:     8
[2021-09-07 20:10:09]: request_log_location: '/Users/Shared/.niohttpd/logs/request.log'
[2021-09-07 20:10:09]: request_log_max_size: 10.49 MB
[2021-09-07 20:10:09]: request_log_max_age:  60 Minutes
[2021-09-07 20:10:09]: mainlog_location:     '/Users/Shared/.niohttpd/logs/mainlog.log'
[2021-09-07 20:10:09]: mainlog_max_size:     10.49 MB
[2021-09-07 20:10:09]: mainlog_max_age:      360 Minutes
[2021-09-07 20:10:09]: stats_log_location:   '/Users/Shared/.niohttpd/logs/stats.log'
[2021-09-07 20:10:09]: stats_log_max_size:   10.49 MB
[2021-09-07 20:10:09]: stats_log_max_age:    360 Minutes
[2021-09-07 20:10:09]: stats_frequency:      10 Seconds
[2021-09-07 20:10:09]: max_log_queue:        100,000
[2021-09-07 20:10:09]: tls_cert_file:        None
[2021-09-07 20:10:09]: tls_private_key_file: None
[2021-09-07 20:10:09]: WARNING! flag set:    'debug'
[2021-09-07 20:10:09]: WARNING! flag set:    'delete_request_rotation'
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-07 20:10:09]: Server Started
```

## Request log

The request log is used to track information about individual requests. Each request is logged to the request log, but the actual logging takes place in another thread. If the server is overloaded, request logging can be dropped. The configured max_log_queue is the number of pending requests that are stored before request data is dropped and not logged to the request log. By default, this number is set to 100,000. The logging thread executes every 100 ms. Under normal circumstances, requests are not dropped with the default configurations. The data that is logged in the request log is configurable. See the confirguration section for more details on how to configure the request log.

A sample output of the mainlog looks like this:

```
|method|uri|query|User-Agent|Referer|ProcTime
[2021-09-07 20:12:36]: |GET|/||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.271693ms
[2021-09-07 20:12:36]: |GET|/lettherebebitcom.png||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/|2.871201ms
[2021-09-07 20:12:36]: |GET|/favicon-32x32.png||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/|0.280963ms
[2021-09-07 20:12:40]: |GET|/asdf||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.230612ms
[2021-09-07 20:12:40]: |GET|/DERP.jpeg||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/asdf|0.603154ms
[2021-09-07 20:12:42]: |GET|/printheaders||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.407074ms
[2021-09-07 20:12:43]: |GET|/favicon.ico||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/printheaders|0.334554ms
[2021-09-07 20:12:48]: |GET|/get_session||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.124589ms
[2021-09-07 20:12:50]: |GET|/set_session|abc=109|Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.119635ms
```

## Statistical log

The statistical log, shows statistics about the internals of the server. It can be used to see if there are slow pages and other information about the state of the server. The statistical log, logs data periodically. Two types of lines are logged.
* 1.) Data about the entire history of the server's current run.
* 2.) Data about the last n seconds of the server's current run, where n is configurable. The default value is every 5 seconds.

The data logged in this file are:
* 1.) REQUESTS - the number of requests (over the last n seconds or the entire history)
* 2.) CONNS - The current number of connections into the server.
* 3.) CONNECTS - The number of connections made to the server (over the last n seconds or the entire history).
* 4.) QPS - Queries per second. The number of queries made to the server per second (over the last n seconds or the entire history).
* 5.) IDLE_DISC - The number of connections which were disconnected after being idle for a period of time (the default is 2 minutes).
* 6.) RTIMEOUT - The number of connections which were disconnected after not making any requests at all for a period of time (the default is 30 seconds).
* 7.) AVG_LAT - The average latency per request (this includes the portion of the request that is either executing the rustlet or reading the file to return to the user for static HTTP requests.
* 8.) MAX_LAT - The maximum latency for a request in milliseconds.

A sample of the statistical log looks like this:

```
Statistical Log V_1.0:         REQUESTS       CONNS        CONNECTS         QPS   IDLE_DISC    RTIMEOUT     AVG_LAT     MAX_LAT
-------------------------------------------------------------------------------------------------------------------------------
      0 Days 00:01:00:        6,802,182          87          68,087   113369.70           0           0   0.0071864   24.933976
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-05 20:13:34]:        1,136,972          87          11,387   113697.20           0           0   0.0069253   13.433473
[2021-09-05 20:13:44]:        1,161,684          99          11,613   116168.40           0           0   0.0071729   23.659854
[2021-09-05 20:13:54]:        1,125,295          34          11,200   112529.50           0           0   0.0072360   14.536791
[2021-09-05 20:14:04]:        1,147,914          31          11,500   114791.40           0           0   0.0070697   13.358339
[2021-09-05 20:14:14]:        1,147,520          43          11,465   114752.00           0           0   0.0073007   13.523405
[2021-09-05 20:14:24]:        1,124,275          39          11,235   112427.50           0           0   0.0073382   15.702629
-------------------------------------------------------------------------------------------------------------------------------
Statistical Log V_1.0:         REQUESTS       CONNS        CONNECTS         QPS   IDLE_DISC    RTIMEOUT     AVG_LAT     MAX_LAT
-------------------------------------------------------------------------------------------------------------------------------
      0 Days 00:02:00:       13,649,810           8         136,500   113748.42           0           0   0.0072068   24.933976
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-05 20:14:34]:        1,140,940           8          11,400   114094.00           0           0   0.0072487   13.600388
[2021-09-05 20:14:44]:        1,143,399          80          11,480   114339.90           0           0   0.0069738   14.146019
[2021-09-05 20:14:54]:        1,146,106          37          11,420   114610.60           0           0   0.0072077   15.134817
[2021-09-05 20:15:04]:        1,142,332          41          11,442   114233.20           0           0   0.0071162   14.670403
[2021-09-05 20:15:14]:        1,126,276          43          11,258   112627.60           0           0   0.0072569   13.363555
[2021-09-05 20:15:24]:        1,143,816          34          11,443   114381.60           0           0   0.0072189   13.484573
-------------------------------------------------------------------------------------------------------------------------------
Statistical Log V_1.0:         REQUESTS       CONNS        CONNECTS         QPS   IDLE_DISC    RTIMEOUT     AVG_LAT     MAX_LAT
-------------------------------------------------------------------------------------------------------------------------------
      0 Days 00:03:00:       20,460,296          43         204,643   113668.31           0           0   0.0072020   24.933976
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-05 20:15:34]:        1,108,557          43          11,100   110855.70           0           0   0.0073872   12.859597
[2021-09-05 20:15:44]:        1,149,210          19          11,457   114921.00           0           0   0.0072592   18.166237
[2021-09-05 20:15:54]:        1,135,703          63          11,395   113570.30           0           0   0.0070510   11.664569
[2021-09-05 20:16:04]:        1,133,285          75          11,305   113328.50           0           0   0.0072662   15.691763
[2021-09-05 20:16:14]:        1,129,587          81          11,300   112958.70           0           0   0.0072674   13.482526
[2021-09-05 20:16:24]:        1,156,421          67          11,574   115642.10           0           0   0.0070367   14.203071
```

# TLS support

TLS is supported. See the documentation mentioned in the configuration section. [Rustls](https://github.com/rustls/rustls) is used for TLS support on all platforms. Performance is only slightly impacted. On the same Linux box used for testing of non-tls, the performance is about 62,000 requests per second compared to about 114,000 requests per second for non-tls. Latency actually improves a bit (probably due to the lower throughput). For convenience, the rustls main.rs adds the tls configuration options.

```
# ./target/release/rustlet --help
```

Here is the output of the statistical log during a run on a 6-core intel cpu:

```
Statistical Log V_1.0:         REQUESTS       CONNS        CONNECTS         QPS   IDLE_DISC    RTIMEOUT     AVG_LAT     MAX_LAT
-------------------------------------------------------------------------------------------------------------------------------
      3 Days 18:02:11:   20,119,048,913          45     201,190,551   62070.733           5           1   0.0109919   4000.7869
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-11 13:10:05]:          623,324          45           6,236   62332.400           0           0   0.0109845   10.740607
[2021-09-11 13:10:15]:          619,026          31           6,188   61902.600           0           0   0.0110722   17.283270
[2021-09-11 13:10:25]:          625,097          83           6,253   62509.700           0           0   0.0109907   14.433026
[2021-09-11 13:10:35]:          620,377          83           6,203   62037.700           0           0   0.0110005   13.617574
[2021-09-11 13:10:45]:          617,451          59           6,174   61745.100           0           0   0.0109883   12.331127
[2021-09-11 13:10:55]:          620,085          67           6,205   62008.500           0           0   0.0109998   11.800872
```

So, as seen above low latency and high throughput (60,000 requests per second on moderately priced ($182) cpu) is possible with rustlets + tls). Here's the output of the benchmark for reference:

```
$ ./target/release/rustlet -c -x 100 -t 100 -i 10 --tls
[2021-09-11 18:52:33]: Running client 
[2021-09-11 18:52:33]: Threads=100
[2021-09-11 18:52:33]: Iterations=10
[2021-09-11 18:52:33]: Requests per thread per iteration=100
--------------------------------------------------------------------------------
[2021-09-11 18:52:33]: Iteration 1 complete. 
[2021-09-11 18:52:34]: Iteration 2 complete. 
[2021-09-11 18:52:34]: Iteration 3 complete. 
[2021-09-11 18:52:34]: Iteration 4 complete. 
[2021-09-11 18:52:34]: Iteration 5 complete. 
[2021-09-11 18:52:34]: Iteration 6 complete. 
[2021-09-11 18:52:34]: Iteration 7 complete. 
[2021-09-11 18:52:35]: Iteration 8 complete. 
[2021-09-11 18:52:35]: Iteration 9 complete. 
[2021-09-11 18:52:35]: Iteration 10 complete. 
--------------------------------------------------------------------------------
[2021-09-11 18:52:35]: Test complete in 1583 ms
[2021-09-11 18:52:35]: QPS=63171.19393556538
[2021-09-11 18:52:35]: Average latency=0.15323981941ms
[2021-09-11 18:52:35]: Max latency=14.611269ms
```

# Configuration

The rustlet container is configured via the [rustlet_init](https://37miners.github.io/rustlet/librustlet/macro.rustlet_init.html) macro. All configuration structs implement the Default trait so the defaults can be used. Also, all of the fields are fully documented in the documentation linked to above.

# Socklets

As of [Release 0.0.2](https://github.com/37miners/rustlet/releases/tag/0.0.2), A Websocket API called [Socklets](https://37miners.github.io/rustlet/librustlet/macro.socklet.html) is supported in the Rustlet project. Socklets are the equivalent of Rustlets for Websockets as opposed to HTTP. A perf tool is also included in the release to benchmark the performance of the Websocket API. This tool can be built by going to the etc/perf directory and following the instructions in the README.md file. The output of one such run is posted below:

```
$ ./target/release/perf -i 10 -x 1000 -t 80 -m 1500 -o
[2022-01-30 00:26:51]: (INFO) Starting Perf test!
------------------------------------------------------------------------------------------
[2022-01-30 00:26:51]: (INFO) Iteration 1 complete. 
[2022-01-30 00:26:51]: (INFO) Iteration 2 complete. 
[2022-01-30 00:26:52]: (INFO) Iteration 3 complete. 
[2022-01-30 00:26:52]: (INFO) Iteration 4 complete. 
[2022-01-30 00:26:53]: (INFO) Iteration 5 complete. 
[2022-01-30 00:26:53]: (INFO) Iteration 6 complete. 
[2022-01-30 00:26:53]: (INFO) Iteration 7 complete. 
[2022-01-30 00:26:54]: (INFO) Iteration 8 complete. 
[2022-01-30 00:26:54]: (INFO) Iteration 9 complete. 
[2022-01-30 00:26:54]: (INFO) Iteration 10 complete. 
------------------------------------------------------------------------------------------
[2022-01-30 00:26:54]: (INFO) Total elapsed time = 3,905 ms. Total messages = 1,600,000.
[2022-01-30 00:26:54]: (INFO) Messages per second = 409731.11.
[2022-01-30 00:26:54]: (INFO) Average round trip latency = 0.35 ms.
------------------------------------------------------------------------------------------
-------------------------------------Latency Histogram------------------------------------
------------------------------------------------------------------------------------------
|===>                                               |  3.797% (0.00ms - 0.07ms) num=30,374
|=============>                                     | 13.857% (0.07ms - 0.15ms) num=110,858
|===================>                               | 19.367% (0.15ms - 0.23ms) num=154,933
|=================>                                 | 17.799% (0.23ms - 0.30ms) num=142,391
|=============>                                     | 13.499% (0.30ms - 0.38ms) num=107,994
|=========>                                         |  9.604% (0.38ms - 0.45ms) num=76,833
|======>                                            |  6.918% (0.45ms - 0.53ms) num=55,344
|====>                                              |  4.765% (0.53ms - 0.60ms) num=38,120
|===>                                               |  3.233% (0.60ms - 0.68ms) num=25,868
|==>                                                |  2.177% (0.68ms - 0.75ms) num=17,417
|=>                                                 |  1.411% (0.75ms - 0.82ms) num=11,292
|>                                                  |  0.947% (0.82ms - 0.90ms) num=7,576
|>                                                  |  0.617% (0.90ms - 0.97ms) num=4,939
|>                                                  |  0.421% (0.97ms - 1.05ms) num=3,371
|>                                                  |  0.281% (1.05ms - 1.12ms) num=2,245
|>                                                  |  0.196% (1.12ms - 1.20ms) num=1,565
|>                                                  |  0.149% (1.20ms - 1.27ms) num=1,195
|>                                                  |  0.116% (1.27ms - 1.35ms) num=929
|>                                                  |  0.087% (1.35ms - 1.43ms) num=695
|>                                                  |  0.758% (1.43ms and up  ) num=6,061
------------------------------------------------------------------------------------------
```

As seen by the numbers above, around 400,000 messages per second can be sent via the Socklet Websocket API. Also, the average latency and other information about latencies are shown in the histogram displayed by the perf tool. In comparison to other [benchmarks](https://blog.feathersjs.com/http-vs-websockets-a-performance-comparison-da2533f13a77), these numbers are quite good (over 100X faster on comparable hardware). In addition to the tests ran on this (a 6-core CPU), another test on a dual core Intel Nuc resulted in around 150,000 messages per second with similar latencies. Socklets are also easy to use as seen in the examples in the documentation.

# Samples

The Rustlet [macro library](https://37miners.github.io/rustlet/librustlet/index.html)  documentation provides numerous working examples. Also, the [rustlet-simple](https://github.com/37miners/rustlet-simple) project shows how to write and deploy a hello world rustlet in 3 easy steps. More examples to come...

