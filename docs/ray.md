install ray in centos
```
   doc:
    https://docs.ray.io/en/latest/ray-overview/index.html

   install  venv：
     python -m venv .env  
     source .env/bin/activate

   change pip source：
     pip config set global.index-url https://pypi.tuna.tsinghua.edu.cn/simple
   
   install ray：
     pip install -U "ray[default]"
   
   run ray:
     ray start --head --dashboard-host='0.0.0.0' --dashboard-port=8265  #header node
     ray start --address='192.168.0.190:6379'    #worker node
     ray status      #watch status
```

first use ray by python
```
  code：
    import ray
    ray.init()

    @ray.remote
    def f(x):
        return x * x

    futures = [f.remote(i) for i in range(4)]
    print(ray.get(futures)) # [0, 1, 4, 9]
    
   tips：
    1.import ray and ray.init() to use it
    2.annotation @ray.remote means i wanna run this function remote
    3.remote() to real use it 
    4.ray.get() is  fetch the result
```

realize plasma 
```
   doc：
      https://arrow.apache.org/docs/python/plasma.html#the-plasma-in-memory-object-store
    
   define：
      In-Memory Distributed Object Store
      every node exists a Object Store,then realize zero-copy data sharing by shared-memory in node
    
   run plasma:
      1.doc show "plasma_store -m 1000000000 -s /tmp/plasma" .
      run it, then display "command not found".
      find the pyarrow install path and it seems to be 'plasma-store-server'
      
      2."Starting object store with directory /dev/shm and huge page support disabled" error
      pause
      
```

process parquet
```
   code:
      import ray
      # Read a directory of files in remote storage.
      ds = ray.data.read_parquet("/home/dataextract/parquet") 
      ds.show(1)
      
   quetsion:
      1.Datasets requires pyarrow >= 6.0.1, < 7.0.0, but 9.0.0 is installed. Reinstall with `pip install -U "pyarrow<7.0.0"`. If you want to disable this pyarrow version check, set the environment variable RAY_DISABLE_PYARROW_VERSION_CHECK=1.
      
      use pip3 install -U "pyarrow<7.0.0"
      
      2.[dataset]: Run `pip install tqdm` to enable progress reporting.
      
      pip3 install tqdm
      
      3. ModuleNotFoundError: No module named '_bz2'
      download _bz2.cpython-38-x86_64-linux-gnu.so and put into /usr/local/python/lib/python3.8/lib-dynload
      
      use command:
      sudo yum install -y bzip2*
      sudo ln -s libbz2.so.1.0.6 libbz2.so.1
      
      4.add repartition：
      import ray
      from multiprocessing import cpu_count
      # Read a directory of files in remote storage.
      ds = ray.data.read_parquet("/home/dataextract/parquet") 
      
      #Repartition the dataset into exactly this number of blocks.
      ds.repartition(cpu_count()).show(4)
 ···
    
   










 
 
