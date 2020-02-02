package mr

import "log"
import "net"
import "os"
import "net/rpc"
import "net/http"
import "sync"
import "time"
import "math/rand"

type MapJob struct {
	No             int
	File           string
	CheckSum       int
	DispatchedTime *time.Time
	Done           bool
}

type ReduceJob struct {
	No             int
	DispatchedTime *time.Time
	CheckSum       int
	Done           bool
}

type Master struct {
	// Your definitions here.

	sync.Mutex

	mapJobs    map[int]*MapJob
	reduceJobs map[int]*ReduceJob
}

// Your code here -- RPC handlers for the worker to call.

func (m *Master) dispatchMapJob() (job *MapJob, allDone bool) {

	allDone = true

	now := time.Now()
	for _, job := range m.mapJobs {
		if job.Done {
			continue
		}

		allDone = false

		if job.DispatchedTime == nil || now.Sub(*job.DispatchedTime) > time.Second*10 {
			job.CheckSum = rand.Int()
			job.DispatchedTime = &now
			return job, allDone
		}
	}

	return nil, allDone
}

func (m *Master) dispatchReduceJob() (job *ReduceJob, allDone bool) {
	allDone = true

	now := time.Now()
	for _, job := range m.reduceJobs {
		if job.Done {
			continue
		}

		allDone = false

		if job.DispatchedTime == nil || now.Sub(*job.DispatchedTime) > time.Second*10 {
			job.CheckSum = rand.Int()
			job.DispatchedTime = &now
			return job, allDone
		}
	}

	return nil, allDone
}

func (m *Master) debug() {
	println("Map jobs:")
	for no, job := range m.mapJobs {
		println(no, " -> ", job.Done)
	}
	println("Reduce jobs:")
	for no, job := range m.reduceJobs {
		println(no, " -> ", job.Done)
	}
}

func (m *Master) Handle(args *RPCArgs, reply *RPCReply) error {
	m.Lock()
	defer m.Unlock()

	// m.debug()
	if args.ReqType != ReqTypeRequestJob {
		if args.ReqType == ReqTypeDoneMapJob && args.CheckSum == m.mapJobs[args.MapNo].CheckSum {
			m.mapJobs[args.MapNo].Done = true
		} else if args.ReqType == ReqTypeDoneReduceJob && args.CheckSum == m.reduceJobs[args.ReduceNo].CheckSum {
			m.reduceJobs[args.ReduceNo].Done = true
		}

		return nil
	}

	reply.TotalReduce = len(m.reduceJobs)
	reply.TotalMap = len(m.mapJobs)

	if job, allDone := m.dispatchMapJob(); job != nil {
		// Dispatch map job
		reply.JobType = JobTypeMap
		reply.MapNo = job.No
		reply.File = job.File
		reply.CheckSum = job.CheckSum
		return nil
	} else if !allDone {
		// Wait for all map jobs
		reply.JobType = JobTypeRetry
		return nil
	}

	if job, allDone := m.dispatchReduceJob(); job != nil {
		// Dispatch reduce job
		reply.JobType = JobTypeReduce
		reply.ReduceNo = job.No
		reply.CheckSum = job.CheckSum
		return nil
	} else if !allDone {
		// Wait for all reduce jobs
		reply.JobType = JobTypeRetry
		return nil
	}

	reply.JobType = JobTypeNone
	return nil
}

//
// an example RPC handler.
//
// the RPC argument and reply types are defined in rpc.go.
//
func (m *Master) Example(args *ExampleArgs, reply *ExampleReply) error {
	reply.Y = args.X + 1
	return nil
}

//
// start a thread that listens for RPCs from worker.go
//
func (m *Master) server() {
	rpc.Register(m)
	rpc.HandleHTTP()
	//l, e := net.Listen("tcp", ":1234")
	os.Remove("mr-socket")
	l, e := net.Listen("unix", "mr-socket")
	if e != nil {
		log.Fatal("listen error:", e)
	}
	go http.Serve(l, nil)
}

//
// main/mrmaster.go calls Done() periodically to find out
// if the entire job has finished.
//
func (m *Master) Done() bool {

	// Your code here.
	m.Lock()
	defer m.Unlock()

	for _, job := range m.mapJobs {
		if !job.Done {
			return false
		}
	}
	for _, job := range m.reduceJobs {
		if !job.Done {
			return false
		}
	}

	return true
}

//
// create a Master.
// main/mrmaster.go calls this function.
//
func MakeMaster(files []string, numOfReduce int) *Master {
	m := Master{}

	// Your code here.
	rand.Seed(time.Now().UnixNano())
	m.mapJobs = make(map[int]*MapJob)
	for no, file := range files {
		m.mapJobs[no] = &MapJob{
			No:   no,
			File: file,
			Done: false,
		}
	}
	m.reduceJobs = make(map[int]*ReduceJob)
	for no := 0; no < numOfReduce; no++ {
		m.reduceJobs[no] = &ReduceJob{
			No:   no,
			Done: false,
		}
	}

	m.server()
	return &m
}
