## Oracle Event Feed

Oracle Event Feed implementation can be found in ./pallets/oracle_event_feed folder.

### Testing Oracle Event Feed

* Only Root user are able to create the event. In this example we use Alice has root user.


### Steps To create event.

* Run the substrate node template.
* Connect [Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944).
* Select the Developer drop down tab in above app then select Extrinsics tab.
* Select the Account as alice.
* choose oracle_event_feed in extrinsics and pass the value in the felids event name and event description.
* Sign the transaction and submit the transaction. once completed you will get notification.
* Choose the chainstate tab under developer drop down tab.
* choose oracleeventfeed function click on submit or [+] symbol.
* you can see the event and will get cleared after 10 secs.

*Note: Only Alice is able to create events,if choose other account it will throw error.