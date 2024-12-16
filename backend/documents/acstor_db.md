# ACStor Database modeling 

- ACStor storage type has the following type: 
  - Nvme
  - Ssd 
  - Azuredisk 
  - San 

- ACStor test feature
  - name, For example, Cluster Restart
  - storage type, this feature is supported by which storage type. 

  A storage type could have many features and an ACStor test feature could be supported by many storage types.

- ACStor workload is defined by 
  - workload name, for example ACStor-snapshot-for-replication-06-nvme
  - for_feature, which ACStor test feature  

  A ACStor workload cover one ACStor test feature.
  A ACStor workload has many ACStor scenario instances.

- ACStor scenario is a running instance for ACStor workload 
  - scenario id 
  - workload definition, reference ACStor workload definition's id
  - subscription  
  - region 
  - rg 
  - aks 
  - arg, used when call acstor.exe



