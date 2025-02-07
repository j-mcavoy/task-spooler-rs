TS(1)                                                                                                                        General Commands Manual                                                                                                                        TS(1)

NAME
       tsp - Task Spooler. A simple unix batch system

SYNOPSIS
       tsp [actions] [options] [command...]

       Actions:  [-KClhVTRqM]  [-t  [id]]  [-c  [id]]  [-p [id]] [-o [id]] [-s [id]] [-r [id]] [-w [id]] [-k [id]] [-u [id]] [-i [id]] [-U <id-id>] [-S [num]] [-a/--get_label [id]] [-F/--full_cmd [id]] [--getenv [var]] [--setenv [var=val]] [--unsetenv [var]] [--get_logdir]
       [--set_logdir [path]] [-M/--serialize [format]]

       Options: [-BnEfzmd] [-L [label]] [-D [id1,id2,...]]  [-W [id1,id2,...]]  [-O [name]] [-N [num]]

DESCRIPTION
       tsp will run by default a per user unix task queue. The user can add commands to the queue, watch that queue at any moment, and look at the task results (actually, standard output and exit error).

SIMPLE USE
       Calling tsp with a command will add that command to the queue, and calling it without commands or parameters will show the task list.

COMMAND OPTIONS
       When adding a job to tsp, we can specify how it will be run and how the results will be collected:

       -n     Do not store the standard output/error in a file at $TMPDIR - let it use the file descriptors decided by the calling process. If it is not used, the jobid for the new task will be output to stdout.

       -z     Pass the output through gzip (only if -n ). Note that the output files will not have a .gz extension.

       -f     Do not put the task into the background. Wait for the command to run without detaching from the terminal. The exit code will be that of the command, and if used together with -n, no result will be stored in the queue.

       -m     Mail the results of the command (output and exit code) to $TS_MAILTO , or to the $USER using /usr/sbin/sendmail.  Look at ENVIRONMENT.

       -L [label]
              Add a label to the task, which will appear next to its command when listing the queue. It makes more comfortable distinguishing similar commands with different goals.

       -d     Run the command only after the last command finished.  It does not depend on how its dependency ends.

       -D [id,...]
              Run the command only after the specified job IDs finished.  It does not depend on how its dependencies end.

       -W [id,...]
              Run the command only if the job of given id finished well (errorlevel = 0). This new task enqueued depends on the result of the previous command. If the task is not run, it is considered as failed for further dependencies.  If the server doesn't have the  job
              id in its list, it will be considered as if the job failed.

       -B     In the case the queue is full (due to TS_MAXCONN or system limits), by default tsp will block the enqueuing command. Using -B, if the queue is full it will exit returning the value 2 instead of blocking.

       -E     Keep  two  different  output files for the command stdout and stderr. stdout goes to the file announced by tsp (look at -o), and stderr goes to the stdout file with an additional ".e". For example, /tmp/ts-out.SKsDw8 and /tmp/ts-out.SKsDw8.e.  Only the stdout
              file gets created with mkstemp, ensuring it does not overwrite any other; the ".e" will be overwritten if it existed.

       -O [name]
              Set the log name to the specified name. Do not include any path in the specified name.

       -N [num]
              Run the command only if there are num slots free in the queue. Without it, the job will run if there is one slot free. For example, if you use the queue to feed cpu cores, and you know that a job will take two cores, with -N you can let tsp know that.

ACTIONS
       Instead of giving a new command, we can use the parameters for other purposes:

       --getenv [var]
              Get the specified environment variable value from the tsp server.

       --setenv [var]
              Set the specified environment variable to the tsp server.

       --unsetenv [var]
              Remove the specified environment variable from the tsp server.

       -K     Kill the tsp server for the calling client. This will remove the unix socket and all the tsp processes related to the queue. This will not kill the command being run at that time.

              It is not reliable to think that tsp -K will finish when the server is really killed. By now it is a race condition.

       -T     Send SIGTERM to all running job groups.

       -C     Clear the results of finished jobs from the queue.

       -l     Show the list of jobs - to be run, running and finished - for the current queue.  This is the default behaviour if tsp is called without options.

       -M/--serialize [format]
              Serialize the job list to the specified format. Choices: {default, json, tab}.

       -q/--last_queue_id
              Show the job ID of the last added.

       -R/--count_running
              Return the number of running jobs

       -a/--get_label [id]
              Show the job label. Of the last added, if not specified.

       -F/--full_cmd [id]
              Show the full command. Of the last added, if not specified.

       --get_logdir
              Show the path containing log files.

       --set_logdir [path]
              Set the path containing log files to the specified path.

       -t [id]
              Show the last ten lines of the output file of the named job, or the last running/run if not specified. If the job is still running, it will keep on showing the additional output until the job finishes. On exit, it returns the errorlevel of the job, as in -c.

       -c [id]
              Run the system's cat to the output file of the named job, or the last running/run if not specified. It will block until all the output can be sent to standard output, and will exit with the job errorlevel as in -c.

       -p [id]
              Show the pid of the named job, or the last running/run if not specified.

       -o [id]
              Show the output file name of the named job, or the last running/run if not specified.

       -s [id]
              Show the job state of the named job, or the last in the queue.

       -r [id]
              Remove the named job, or the last in the queue.

       -w [id]
              Wait for the named job, or for the last in the queue.

       -k [id]
              Kill the process group of the named job (SIGTERM), or the last running/run job if not specified.  Equivalent to kill -- -‘tsp -p‘

       -u [id]
              Make the named job (or the last in the queue) urgent - this means that it goes forward in the queue so it can run as soon as possible.

       -i [id]
              Show information about the named job (or the last run). It will show the command line, some times related to the task, and also any information resulting from TS_ENV (Look at ENVIRONMENT).

       -U <id-id>
              Interchange the queue positions of the named jobs (separated by a hyphen and no spaces).

       -h     Show help on standard output.

       -V     Show the program version.

MULTI-SLOT
       tsp by default offers a queue where each job runs only after the previous finished.  Nevertheless, you can change the maximum number of jobs running at once with the -S [num] parameter. We call that number the amount of slots. You can also set the initial number  of
       jobs  with the environment variable TS_SLOTS .  When increasing this setting, queued waiting jobs will be run at once until reaching the maximum set. When decreasing this setting, no other job will be run until it can meet the amount of running jobs set.  When using
       an amount of slots greater than 1, the action of some commands may change a bit. For example, -t without jobid will tail the first job running, and -d will try to set the dependency with the last job added.

       -S [num]
              Set the maximum amount of running jobs at once. If you don't specify num it will return the maximum amount of running jobs set.

ENVIRONMENT
       TS_MAXFINISHED
              Limit the number of job results (finished tasks) you want in the queue. Use this option if you are tired of -C.

       TS_MAXCONN
              The maximum number of tsp server connections to clients. This will make the tsp clients block until connections are freed. This helps, for example, on systems with a limited number of processes, because each job waiting in the queue remains as a process. This
              variable has to be set at server start, and cannot be modified later.

       TS_ONFINISH
              If the variable exists pointing to an executable, it will be run by the client after the queued job. It uses execlp, so PATH is used if there are no slashes in the variable content. The executable is run with four parameters: jobid errorlevel  output_filename
              and command.

       TMPDIR As the program output and the unix socket are thought to be stored in a temporary directory, TMPDIR will be used if defined, or /tmp otherwise.

       TS_SOCKET
              Each  queue has a related unix socket. You can specify the socket path with this environment variable. This way, you can have a queue for your heavy disk operations, another for heavy use of ram., and have a simple script/alias wrapper over tsp for those spe‐
              cial queues. If it is not specified, it will be $TMPDIR/socket-ts.[uid].

       TS_SLOTS
              Set the number of slots at the start of the server, similar to -S, but the contents of the variable are read only when running the first instance of tsp.

       TS_MAILTO
              Send the letters with job results to the address specified in this variable.  Otherwise, they are sent to $USER or if not defined, nobody.  The system /usr/sbin/sendmail is used. The job outputs are not sent as an attachment, so understand the consequences if
              you use the -gm flags together.

       USER   As seen above, it is used for the mail destination if TS_MAILTO is not specified.

       TS_SAVELIST
              If it is defined when starting the queue server (probably the first tsp command run), on SIGTERM the queue status will be saved to the file pointed by this environment variable - for example, at system shutdown.

       TS_ENV This has a command to be run at enqueue time through /bin/sh. The output of the command will be readable through the option -i. You can use a command which shows relevant environment for the command run.  For example, you may use TS_ENV='pwd;set;mount'.

FILES
       /tmp/ts.error
              if tsp finds any internal problem, you should find an error report there.  Please send this to the author as part of the bug report.

BUGS
       tsp expects a simple command line. It does not start a shell parser.  If you want to run complex shell commands, you may want to run them through sh -c 'commands...'  Also, remember that stdin/stdout/stderr will be detached, so do not use  your  shell's  redirection
       operators when you put a job into background.  You can use them inside the sh -c in order to set redirections to the command run.

       If an internal problem is found in runtime, a file /tmp/ts.error is created, which you can submit to the developer in order to fix the bug.

SEE ALSO
       at(1)

AUTHOR
       Duc Nguyen and Lluis Batlle i Rossell

NOTES
       This page describes tsp as in version 31dc8b7. Other versions may differ. The file TRICKS found in the distribution package can show some ideas on special uses of tsp.

Task Spooler 31dc8b7                                                                                                                 2025-01                                                                                                                                TS(1)
