# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [watcher/addresses/v1/addresses.proto](#watcher_addresses_v1_addresses-proto)
    - [Address](#watcher-addresses-v1-Address)
    - [AppendAddressesToWatchListRequest](#watcher-addresses-v1-AppendAddressesToWatchListRequest)
    - [AppendAddressesToWatchListResponse](#watcher-addresses-v1-AppendAddressesToWatchListResponse)
    - [UpdateWatchListRequest](#watcher-addresses-v1-UpdateWatchListRequest)
    - [UpdateWatchListResponse](#watcher-addresses-v1-UpdateWatchListResponse)
  
    - [AddressesService](#watcher-addresses-v1-AddressesService)
  
- [watcher/subscriber/v1/subscriber.proto](#watcher_subscriber_v1_subscriber-proto)
    - [SubscribeMempoolRequest](#watcher-subscriber-v1-SubscribeMempoolRequest)
    - [SubscribeMempoolResponse](#watcher-subscriber-v1-SubscribeMempoolResponse)
  
    - [SubscriberService](#watcher-subscriber-v1-SubscriberService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="watcher_addresses_v1_addresses-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## watcher/addresses/v1/addresses.proto



<a name="watcher-addresses-v1-Address"></a>

### Address



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [string](#string) |  |  |
| blockchain | [eproxy.common.v2.Blockchain](#eproxy-common-v2-Blockchain) |  |  |






<a name="watcher-addresses-v1-AppendAddressesToWatchListRequest"></a>

### AppendAddressesToWatchListRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| addresses | [Address](#watcher-addresses-v1-Address) | repeated |  |






<a name="watcher-addresses-v1-AppendAddressesToWatchListResponse"></a>

### AppendAddressesToWatchListResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| addresses | [Address](#watcher-addresses-v1-Address) | repeated |  |






<a name="watcher-addresses-v1-UpdateWatchListRequest"></a>

### UpdateWatchListRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| addresses | [Address](#watcher-addresses-v1-Address) | repeated |  |






<a name="watcher-addresses-v1-UpdateWatchListResponse"></a>

### UpdateWatchListResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| addresses | [Address](#watcher-addresses-v1-Address) | repeated |  |





 

 

 


<a name="watcher-addresses-v1-AddressesService"></a>

### AddressesService
Service which interacts with addresses

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| UpdateWatchList | [UpdateWatchListRequest](#watcher-addresses-v1-UpdateWatchListRequest) | [UpdateWatchListResponse](#watcher-addresses-v1-UpdateWatchListResponse) | Update client addresses watch list |
| AppendAddressesToWatchList | [AppendAddressesToWatchListRequest](#watcher-addresses-v1-AppendAddressesToWatchListRequest) | [AppendAddressesToWatchListResponse](#watcher-addresses-v1-AppendAddressesToWatchListResponse) | Append new addresses to watch list without replace |

 



<a name="watcher_subscriber_v1_subscriber-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## watcher/subscriber/v1/subscriber.proto



<a name="watcher-subscriber-v1-SubscribeMempoolRequest"></a>

### SubscribeMempoolRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| blockchain | [eproxy.common.v2.Blockchain](#eproxy-common-v2-Blockchain) |  |  |






<a name="watcher-subscriber-v1-SubscribeMempoolResponse"></a>

### SubscribeMempoolResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [eproxy.transactions.v2.Transaction](#eproxy-transactions-v2-Transaction) |  |  |
| ping | [string](#string) |  |  |





 

 

 


<a name="watcher-subscriber-v1-SubscriberService"></a>

### SubscriberService
Service which provides subscriber api

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SubscribeMempool | [SubscribeMempoolRequest](#watcher-subscriber-v1-SubscribeMempoolRequest) | [SubscribeMempoolResponse](#watcher-subscriber-v1-SubscribeMempoolResponse) stream | Subscribe mempool transactions |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

