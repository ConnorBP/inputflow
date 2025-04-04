# Input Flow QMP
This plugin enables controlling the mouse of your QEMU Virtual Machine instances via the `QMP` protocol. Currently tcp mode is preferred over unix sockets as unix sockets might have access errors when multiple devices try to acess it (i.e. memflow)

# QEMU Configuration
On QEMU via libvirt you can pass through the qmp configuration via a command line option to qemu like so:
```xml
<domain>
  ...
  <qemu:commandline>
    <qemu:arg value="-qmp"/>
    <qemu:arg value="tcp:127.0.0.1:6448,server,nowait"/>
  </qemu:commandline>
</domain>
```

or you can directly pass the parameter to qemu otherwise: `-qmp tcp:127.0.0.1:6448,server,nowait`

