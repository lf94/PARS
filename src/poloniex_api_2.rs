#![allow(non_snake_case)]

extern crate chrono;
extern crate circular_queue;

use chrono::{
  Duration,
  Local
};
use circular_queue::{
  CircularQueue
};
use serde_json;

use std::collections::HashMap;
use serde_json::{ 
  Value
};

const MAX_CAPACITY: usize = 10;

const MARKETS_JSON: &str = r#"{ "byCurrencyPair":{"BTC_BCN":{"id":7,"baseID":28,"quoteID":17,"base":"BTC","quote":"BCN","currencyPair":"BTC_BCN"},"BTC_BELA":{"id":8,"baseID":28,"quoteID":20,"base":"BTC","quote":"BELA","currencyPair":"BTC_BELA"},"BTC_BLK":{"id":10,"baseID":28,"quoteID":22,"base":"BTC","quote":"BLK","currencyPair":"BTC_BLK"},"BTC_BTCD":{"id":12,"baseID":28,"quoteID":29,"base":"BTC","quote":"BTCD","currencyPair":"BTC_BTCD"},"BTC_BTM":{"id":13,"baseID":28,"quoteID":31,"base":"BTC","quote":"BTM","currencyPair":"BTC_BTM"},"BTC_BTS":{"id":14,"baseID":28,"quoteID":32,"base":"BTC","quote":"BTS","currencyPair":"BTC_BTS"},"BTC_BURST":{"id":15,"baseID":28,"quoteID":34,"base":"BTC","quote":"BURST","currencyPair":"BTC_BURST"},"BTC_CLAM":{"id":20,"baseID":28,"quoteID":43,"base":"BTC","quote":"CLAM","currencyPair":"BTC_CLAM"},"BTC_DGB":{"id":25,"baseID":28,"quoteID":53,"base":"BTC","quote":"DGB","currencyPair":"BTC_DGB"},"BTC_DOGE":{"id":27,"baseID":28,"quoteID":59,"base":"BTC","quote":"DOGE","currencyPair":"BTC_DOGE"},"BTC_DASH":{"id":24,"baseID":28,"quoteID":60,"base":"BTC","quote":"DASH","currencyPair":"BTC_DASH"},"BTC_EMC2":{"id":28,"baseID":28,"quoteID":69,"base":"BTC","quote":"EMC2","currencyPair":"BTC_EMC2"},"BTC_FLDC":{"id":31,"baseID":28,"quoteID":78,"base":"BTC","quote":"FLDC","currencyPair":"BTC_FLDC"},"BTC_GAME":{"id":38,"baseID":28,"quoteID":93,"base":"BTC","quote":"GAME","currencyPair":"BTC_GAME"},"BTC_HUC":{"id":43,"baseID":28,"quoteID":105,"base":"BTC","quote":"HUC","currencyPair":"BTC_HUC"},"BTC_LTC":{"id":50,"baseID":28,"quoteID":125,"base":"BTC","quote":"LTC","currencyPair":"BTC_LTC"},"BTC_MAID":{"id":51,"baseID":28,"quoteID":127,"base":"BTC","quote":"MAID","currencyPair":"BTC_MAID"},"BTC_OMNI":{"id":58,"baseID":28,"quoteID":143,"base":"BTC","quote":"OMNI","currencyPair":"BTC_OMNI"},"BTC_NAUT":{"id":60,"baseID":28,"quoteID":150,"base":"BTC","quote":"NAUT","currencyPair":"BTC_NAUT"},"BTC_NAV":{"id":61,"baseID":28,"quoteID":151,"base":"BTC","quote":"NAV","currencyPair":"BTC_NAV"},"BTC_NEOS":{"id":63,"baseID":28,"quoteID":153,"base":"BTC","quote":"NEOS","currencyPair":"BTC_NEOS"},"BTC_NMC":{"id":64,"baseID":28,"quoteID":155,"base":"BTC","quote":"NMC","currencyPair":"BTC_NMC"},"BTC_NOTE":{"id":66,"baseID":28,"quoteID":157,"base":"BTC","quote":"NOTE","currencyPair":"BTC_NOTE"},"BTC_NXT":{"id":69,"baseID":28,"quoteID":162,"base":"BTC","quote":"NXT","currencyPair":"BTC_NXT"},"BTC_PINK":{"id":73,"baseID":28,"quoteID":168,"base":"BTC","quote":"PINK","currencyPair":"BTC_PINK"},"BTC_POT":{"id":74,"baseID":28,"quoteID":171,"base":"BTC","quote":"POT","currencyPair":"BTC_POT"},"BTC_PPC":{"id":75,"baseID":28,"quoteID":172,"base":"BTC","quote":"PPC","currencyPair":"BTC_PPC"},"BTC_RIC":{"id":83,"baseID":28,"quoteID":183,"base":"BTC","quote":"RIC","currencyPair":"BTC_RIC"},"BTC_SJCX":{"id":86,"baseID":28,"quoteID":189,"base":"BTC","quote":"SJCX","currencyPair":"BTC_SJCX"},"BTC_STR":{"id":89,"baseID":28,"quoteID":198,"base":"BTC","quote":"STR","currencyPair":"BTC_STR"},"BTC_SYS":{"id":92,"baseID":28,"quoteID":204,"base":"BTC","quote":"SYS","currencyPair":"BTC_SYS"},"BTC_VIA":{"id":97,"baseID":28,"quoteID":218,"base":"BTC","quote":"VIA","currencyPair":"BTC_VIA"},"BTC_VRC":{"id":99,"baseID":28,"quoteID":220,"base":"BTC","quote":"VRC","currencyPair":"BTC_VRC"},"BTC_VTC":{"id":100,"baseID":28,"quoteID":221,"base":"BTC","quote":"VTC","currencyPair":"BTC_VTC"},"BTC_XBC":{"id":104,"baseID":28,"quoteID":229,"base":"BTC","quote":"XBC","currencyPair":"BTC_XBC"},"BTC_XCP":{"id":108,"baseID":28,"quoteID":233,"base":"BTC","quote":"XCP","currencyPair":"BTC_XCP"},"BTC_XMR":{"id":114,"baseID":28,"quoteID":240,"base":"BTC","quote":"XMR","currencyPair":"BTC_XMR"},"BTC_XPM":{"id":116,"baseID":28,"quoteID":242,"base":"BTC","quote":"XPM","currencyPair":"BTC_XPM"},"BTC_XRP":{"id":117,"baseID":28,"quoteID":243,"base":"BTC","quote":"XRP","currencyPair":"BTC_XRP"},"BTC_XVC":{"id":98,"baseID":28,"quoteID":253,"base":"BTC","quote":"XVC","currencyPair":"BTC_XVC"},"BTC_FLO":{"id":32,"baseID":28,"quoteID":254,"base":"BTC","quote":"FLO","currencyPair":"BTC_FLO"},"BTC_XEM":{"id":112,"baseID":28,"quoteID":256,"base":"BTC","quote":"XEM","currencyPair":"BTC_XEM"},"BTC_GRC":{"id":40,"baseID":28,"quoteID":261,"base":"BTC","quote":"GRC","currencyPair":"BTC_GRC"},"BTC_ETH":{"id":148,"baseID":28,"quoteID":267,"base":"BTC","quote":"ETH","currencyPair":"BTC_ETH"},"BTC_SC":{"id":150,"baseID":28,"quoteID":268,"base":"BTC","quote":"SC","currencyPair":"BTC_SC"},"BTC_BCY":{"id":151,"baseID":28,"quoteID":269,"base":"BTC","quote":"BCY","currencyPair":"BTC_BCY"},"BTC_EXP":{"id":153,"baseID":28,"quoteID":270,"base":"BTC","quote":"EXP","currencyPair":"BTC_EXP"},"BTC_FCT":{"id":155,"baseID":28,"quoteID":271,"base":"BTC","quote":"FCT","currencyPair":"BTC_FCT"},"BTC_RADS":{"id":158,"baseID":28,"quoteID":274,"base":"BTC","quote":"RADS","currencyPair":"BTC_RADS"},"BTC_AMP":{"id":160,"baseID":28,"quoteID":275,"base":"BTC","quote":"AMP","currencyPair":"BTC_AMP"},"BTC_DCR":{"id":162,"baseID":28,"quoteID":277,"base":"BTC","quote":"DCR","currencyPair":"BTC_DCR"},"BTC_LSK":{"id":163,"baseID":28,"quoteID":278,"base":"BTC","quote":"LSK","currencyPair":"BTC_LSK"},"BTC_LBC":{"id":167,"baseID":28,"quoteID":280,"base":"BTC","quote":"LBC","currencyPair":"BTC_LBC"},"BTC_STEEM":{"id":168,"baseID":28,"quoteID":281,"base":"BTC","quote":"STEEM","currencyPair":"BTC_STEEM"},"BTC_SBD":{"id":170,"baseID":28,"quoteID":282,"base":"BTC","quote":"SBD","currencyPair":"BTC_SBD"},"BTC_ETC":{"id":171,"baseID":28,"quoteID":283,"base":"BTC","quote":"ETC","currencyPair":"BTC_ETC"},"BTC_REP":{"id":174,"baseID":28,"quoteID":284,"base":"BTC","quote":"REP","currencyPair":"BTC_REP"},"BTC_ARDR":{"id":177,"baseID":28,"quoteID":285,"base":"BTC","quote":"ARDR","currencyPair":"BTC_ARDR"},"BTC_ZEC":{"id":178,"baseID":28,"quoteID":286,"base":"BTC","quote":"ZEC","currencyPair":"BTC_ZEC"},"BTC_STRAT":{"id":182,"baseID":28,"quoteID":287,"base":"BTC","quote":"STRAT","currencyPair":"BTC_STRAT"},"BTC_NXC":{"id":183,"baseID":28,"quoteID":288,"base":"BTC","quote":"NXC","currencyPair":"BTC_NXC"},"BTC_PASC":{"id":184,"baseID":28,"quoteID":289,"base":"BTC","quote":"PASC","currencyPair":"BTC_PASC"},"BTC_GNT":{"id":185,"baseID":28,"quoteID":290,"base":"BTC","quote":"GNT","currencyPair":"BTC_GNT"},"BTC_GNO":{"id":187,"baseID":28,"quoteID":291,"base":"BTC","quote":"GNO","currencyPair":"BTC_GNO"},"BTC_BCH":{"id":189,"baseID":28,"quoteID":292,"base":"BTC","quote":"BCH","currencyPair":"BTC_BCH"},"BTC_ZRX":{"id":192,"baseID":28,"quoteID":293,"base":"BTC","quote":"ZRX","currencyPair":"BTC_ZRX"},"BTC_CVC":{"id":194,"baseID":28,"quoteID":294,"base":"BTC","quote":"CVC","currencyPair":"BTC_CVC"},"BTC_OMG":{"id":196,"baseID":28,"quoteID":295,"base":"BTC","quote":"OMG","currencyPair":"BTC_OMG"},"BTC_GAS":{"id":198,"baseID":28,"quoteID":296,"base":"BTC","quote":"GAS","currencyPair":"BTC_GAS"},"BTC_STORJ":{"id":200,"baseID":28,"quoteID":297,"base":"BTC","quote":"STORJ","currencyPair":"BTC_STORJ"},"USDT_BTC":{"id":121,"baseID":214,"quoteID":28,"base":"USDT","quote":"BTC","currencyPair":"USDT_BTC"},"USDT_DASH":{"id":122,"baseID":214,"quoteID":60,"base":"USDT","quote":"DASH","currencyPair":"USDT_DASH"},"USDT_LTC":{"id":123,"baseID":214,"quoteID":125,"base":"USDT","quote":"LTC","currencyPair":"USDT_LTC"},"USDT_NXT":{"id":124,"baseID":214,"quoteID":162,"base":"USDT","quote":"NXT","currencyPair":"USDT_NXT"},"USDT_STR":{"id":125,"baseID":214,"quoteID":198,"base":"USDT","quote":"STR","currencyPair":"USDT_STR"},"USDT_XMR":{"id":126,"baseID":214,"quoteID":240,"base":"USDT","quote":"XMR","currencyPair":"USDT_XMR"},"USDT_XRP":{"id":127,"baseID":214,"quoteID":243,"base":"USDT","quote":"XRP","currencyPair":"USDT_XRP"},"USDT_ETH":{"id":149,"baseID":214,"quoteID":267,"base":"USDT","quote":"ETH","currencyPair":"USDT_ETH"},"USDT_ETC":{"id":173,"baseID":214,"quoteID":283,"base":"USDT","quote":"ETC","currencyPair":"USDT_ETC"},"USDT_REP":{"id":175,"baseID":214,"quoteID":284,"base":"USDT","quote":"REP","currencyPair":"USDT_REP"},"USDT_ZEC":{"id":180,"baseID":214,"quoteID":286,"base":"USDT","quote":"ZEC","currencyPair":"USDT_ZEC"},"USDT_BCH":{"id":191,"baseID":214,"quoteID":292,"base":"USDT","quote":"BCH","currencyPair":"USDT_BCH"},"XMR_BCN":{"id":129,"baseID":240,"quoteID":17,"base":"XMR","quote":"BCN","currencyPair":"XMR_BCN"},"XMR_BLK":{"id":130,"baseID":240,"quoteID":22,"base":"XMR","quote":"BLK","currencyPair":"XMR_BLK"},"XMR_BTCD":{"id":131,"baseID":240,"quoteID":29,"base":"XMR","quote":"BTCD","currencyPair":"XMR_BTCD"},"XMR_DASH":{"id":132,"baseID":240,"quoteID":60,"base":"XMR","quote":"DASH","currencyPair":"XMR_DASH"},"XMR_LTC":{"id":137,"baseID":240,"quoteID":125,"base":"XMR","quote":"LTC","currencyPair":"XMR_LTC"},"XMR_MAID":{"id":138,"baseID":240,"quoteID":127,"base":"XMR","quote":"MAID","currencyPair":"XMR_MAID"},"XMR_NXT":{"id":140,"baseID":240,"quoteID":162,"base":"XMR","quote":"NXT","currencyPair":"XMR_NXT"},"XMR_ZEC":{"id":181,"baseID":240,"quoteID":286,"base":"XMR","quote":"ZEC","currencyPair":"XMR_ZEC"},"ETH_LSK":{"id":166,"baseID":267,"quoteID":278,"base":"ETH","quote":"LSK","currencyPair":"ETH_LSK"},"ETH_STEEM":{"id":169,"baseID":267,"quoteID":281,"base":"ETH","quote":"STEEM","currencyPair":"ETH_STEEM"},"ETH_ETC":{"id":172,"baseID":267,"quoteID":283,"base":"ETH","quote":"ETC","currencyPair":"ETH_ETC"},"ETH_REP":{"id":176,"baseID":267,"quoteID":284,"base":"ETH","quote":"REP","currencyPair":"ETH_REP"},"ETH_ZEC":{"id":179,"baseID":267,"quoteID":286,"base":"ETH","quote":"ZEC","currencyPair":"ETH_ZEC"},"ETH_GNT":{"id":186,"baseID":267,"quoteID":290,"base":"ETH","quote":"GNT","currencyPair":"ETH_GNT"},"ETH_GNO":{"id":188,"baseID":267,"quoteID":291,"base":"ETH","quote":"GNO","currencyPair":"ETH_GNO"},"ETH_BCH":{"id":190,"baseID":267,"quoteID":292,"base":"ETH","quote":"BCH","currencyPair":"ETH_BCH"},"ETH_ZRX":{"id":193,"baseID":267,"quoteID":293,"base":"ETH","quote":"ZRX","currencyPair":"ETH_ZRX"},"ETH_CVC":{"id":195,"baseID":267,"quoteID":294,"base":"ETH","quote":"CVC","currencyPair":"ETH_CVC"},"ETH_OMG":{"id":197,"baseID":267,"quoteID":295,"base":"ETH","quote":"OMG","currencyPair":"ETH_OMG"},"ETH_GAS":{"id":199,"baseID":267,"quoteID":296,"base":"ETH","quote":"GAS","currencyPair":"ETH_GAS"}},"byID":{"7":{"id":7,"baseID":28,"quoteID":17,"base":"BTC","quote":"BCN","currencyPair":"BTC_BCN"},"8":{"id":8,"baseID":28,"quoteID":20,"base":"BTC","quote":"BELA","currencyPair":"BTC_BELA"},"10":{"id":10,"baseID":28,"quoteID":22,"base":"BTC","quote":"BLK","currencyPair":"BTC_BLK"},"12":{"id":12,"baseID":28,"quoteID":29,"base":"BTC","quote":"BTCD","currencyPair":"BTC_BTCD"},"13":{"id":13,"baseID":28,"quoteID":31,"base":"BTC","quote":"BTM","currencyPair":"BTC_BTM"},"14":{"id":14,"baseID":28,"quoteID":32,"base":"BTC","quote":"BTS","currencyPair":"BTC_BTS"},"15":{"id":15,"baseID":28,"quoteID":34,"base":"BTC","quote":"BURST","currencyPair":"BTC_BURST"},"20":{"id":20,"baseID":28,"quoteID":43,"base":"BTC","quote":"CLAM","currencyPair":"BTC_CLAM"},"25":{"id":25,"baseID":28,"quoteID":53,"base":"BTC","quote":"DGB","currencyPair":"BTC_DGB"},"27":{"id":27,"baseID":28,"quoteID":59,"base":"BTC","quote":"DOGE","currencyPair":"BTC_DOGE"},"24":{"id":24,"baseID":28,"quoteID":60,"base":"BTC","quote":"DASH","currencyPair":"BTC_DASH"},"28":{"id":28,"baseID":28,"quoteID":69,"base":"BTC","quote":"EMC2","currencyPair":"BTC_EMC2"},"31":{"id":31,"baseID":28,"quoteID":78,"base":"BTC","quote":"FLDC","currencyPair":"BTC_FLDC"},"38":{"id":38,"baseID":28,"quoteID":93,"base":"BTC","quote":"GAME","currencyPair":"BTC_GAME"},"43":{"id":43,"baseID":28,"quoteID":105,"base":"BTC","quote":"HUC","currencyPair":"BTC_HUC"},"50":{"id":50,"baseID":28,"quoteID":125,"base":"BTC","quote":"LTC","currencyPair":"BTC_LTC"},"51":{"id":51,"baseID":28,"quoteID":127,"base":"BTC","quote":"MAID","currencyPair":"BTC_MAID"},"58":{"id":58,"baseID":28,"quoteID":143,"base":"BTC","quote":"OMNI","currencyPair":"BTC_OMNI"},"60":{"id":60,"baseID":28,"quoteID":150,"base":"BTC","quote":"NAUT","currencyPair":"BTC_NAUT"},"61":{"id":61,"baseID":28,"quoteID":151,"base":"BTC","quote":"NAV","currencyPair":"BTC_NAV"},"63":{"id":63,"baseID":28,"quoteID":153,"base":"BTC","quote":"NEOS","currencyPair":"BTC_NEOS"},"64":{"id":64,"baseID":28,"quoteID":155,"base":"BTC","quote":"NMC","currencyPair":"BTC_NMC"},"66":{"id":66,"baseID":28,"quoteID":157,"base":"BTC","quote":"NOTE","currencyPair":"BTC_NOTE"},"69":{"id":69,"baseID":28,"quoteID":162,"base":"BTC","quote":"NXT","currencyPair":"BTC_NXT"},"73":{"id":73,"baseID":28,"quoteID":168,"base":"BTC","quote":"PINK","currencyPair":"BTC_PINK"},"74":{"id":74,"baseID":28,"quoteID":171,"base":"BTC","quote":"POT","currencyPair":"BTC_POT"},"75":{"id":75,"baseID":28,"quoteID":172,"base":"BTC","quote":"PPC","currencyPair":"BTC_PPC"},"83":{"id":83,"baseID":28,"quoteID":183,"base":"BTC","quote":"RIC","currencyPair":"BTC_RIC"},"86":{"id":86,"baseID":28,"quoteID":189,"base":"BTC","quote":"SJCX","currencyPair":"BTC_SJCX"},"89":{"id":89,"baseID":28,"quoteID":198,"base":"BTC","quote":"STR","currencyPair":"BTC_STR"},"92":{"id":92,"baseID":28,"quoteID":204,"base":"BTC","quote":"SYS","currencyPair":"BTC_SYS"},"97":{"id":97,"baseID":28,"quoteID":218,"base":"BTC","quote":"VIA","currencyPair":"BTC_VIA"},"99":{"id":99,"baseID":28,"quoteID":220,"base":"BTC","quote":"VRC","currencyPair":"BTC_VRC"},"100":{"id":100,"baseID":28,"quoteID":221,"base":"BTC","quote":"VTC","currencyPair":"BTC_VTC"},"104":{"id":104,"baseID":28,"quoteID":229,"base":"BTC","quote":"XBC","currencyPair":"BTC_XBC"},"108":{"id":108,"baseID":28,"quoteID":233,"base":"BTC","quote":"XCP","currencyPair":"BTC_XCP"},"114":{"id":114,"baseID":28,"quoteID":240,"base":"BTC","quote":"XMR","currencyPair":"BTC_XMR"},"116":{"id":116,"baseID":28,"quoteID":242,"base":"BTC","quote":"XPM","currencyPair":"BTC_XPM"},"117":{"id":117,"baseID":28,"quoteID":243,"base":"BTC","quote":"XRP","currencyPair":"BTC_XRP"},"98":{"id":98,"baseID":28,"quoteID":253,"base":"BTC","quote":"XVC","currencyPair":"BTC_XVC"},"32":{"id":32,"baseID":28,"quoteID":254,"base":"BTC","quote":"FLO","currencyPair":"BTC_FLO"},"112":{"id":112,"baseID":28,"quoteID":256,"base":"BTC","quote":"XEM","currencyPair":"BTC_XEM"},"40":{"id":40,"baseID":28,"quoteID":261,"base":"BTC","quote":"GRC","currencyPair":"BTC_GRC"},"148":{"id":148,"baseID":28,"quoteID":267,"base":"BTC","quote":"ETH","currencyPair":"BTC_ETH"},"150":{"id":150,"baseID":28,"quoteID":268,"base":"BTC","quote":"SC","currencyPair":"BTC_SC"},"151":{"id":151,"baseID":28,"quoteID":269,"base":"BTC","quote":"BCY","currencyPair":"BTC_BCY"},"153":{"id":153,"baseID":28,"quoteID":270,"base":"BTC","quote":"EXP","currencyPair":"BTC_EXP"},"155":{"id":155,"baseID":28,"quoteID":271,"base":"BTC","quote":"FCT","currencyPair":"BTC_FCT"},"158":{"id":158,"baseID":28,"quoteID":274,"base":"BTC","quote":"RADS","currencyPair":"BTC_RADS"},"160":{"id":160,"baseID":28,"quoteID":275,"base":"BTC","quote":"AMP","currencyPair":"BTC_AMP"},"162":{"id":162,"baseID":28,"quoteID":277,"base":"BTC","quote":"DCR","currencyPair":"BTC_DCR"},"163":{"id":163,"baseID":28,"quoteID":278,"base":"BTC","quote":"LSK","currencyPair":"BTC_LSK"},"167":{"id":167,"baseID":28,"quoteID":280,"base":"BTC","quote":"LBC","currencyPair":"BTC_LBC"},"168":{"id":168,"baseID":28,"quoteID":281,"base":"BTC","quote":"STEEM","currencyPair":"BTC_STEEM"},"170":{"id":170,"baseID":28,"quoteID":282,"base":"BTC","quote":"SBD","currencyPair":"BTC_SBD"},"171":{"id":171,"baseID":28,"quoteID":283,"base":"BTC","quote":"ETC","currencyPair":"BTC_ETC"},"174":{"id":174,"baseID":28,"quoteID":284,"base":"BTC","quote":"REP","currencyPair":"BTC_REP"},"177":{"id":177,"baseID":28,"quoteID":285,"base":"BTC","quote":"ARDR","currencyPair":"BTC_ARDR"},"178":{"id":178,"baseID":28,"quoteID":286,"base":"BTC","quote":"ZEC","currencyPair":"BTC_ZEC"},"182":{"id":182,"baseID":28,"quoteID":287,"base":"BTC","quote":"STRAT","currencyPair":"BTC_STRAT"},"183":{"id":183,"baseID":28,"quoteID":288,"base":"BTC","quote":"NXC","currencyPair":"BTC_NXC"},"184":{"id":184,"baseID":28,"quoteID":289,"base":"BTC","quote":"PASC","currencyPair":"BTC_PASC"},"185":{"id":185,"baseID":28,"quoteID":290,"base":"BTC","quote":"GNT","currencyPair":"BTC_GNT"},"187":{"id":187,"baseID":28,"quoteID":291,"base":"BTC","quote":"GNO","currencyPair":"BTC_GNO"},"189":{"id":189,"baseID":28,"quoteID":292,"base":"BTC","quote":"BCH","currencyPair":"BTC_BCH"},"192":{"id":192,"baseID":28,"quoteID":293,"base":"BTC","quote":"ZRX","currencyPair":"BTC_ZRX"},"194":{"id":194,"baseID":28,"quoteID":294,"base":"BTC","quote":"CVC","currencyPair":"BTC_CVC"},"196":{"id":196,"baseID":28,"quoteID":295,"base":"BTC","quote":"OMG","currencyPair":"BTC_OMG"},"198":{"id":198,"baseID":28,"quoteID":296,"base":"BTC","quote":"GAS","currencyPair":"BTC_GAS"},"200":{"id":200,"baseID":28,"quoteID":297,"base":"BTC","quote":"STORJ","currencyPair":"BTC_STORJ"},"121":{"id":121,"baseID":214,"quoteID":28,"base":"USDT","quote":"BTC","currencyPair":"USDT_BTC"},"122":{"id":122,"baseID":214,"quoteID":60,"base":"USDT","quote":"DASH","currencyPair":"USDT_DASH"},"123":{"id":123,"baseID":214,"quoteID":125,"base":"USDT","quote":"LTC","currencyPair":"USDT_LTC"},"124":{"id":124,"baseID":214,"quoteID":162,"base":"USDT","quote":"NXT","currencyPair":"USDT_NXT"},"125":{"id":125,"baseID":214,"quoteID":198,"base":"USDT","quote":"STR","currencyPair":"USDT_STR"},"126":{"id":126,"baseID":214,"quoteID":240,"base":"USDT","quote":"XMR","currencyPair":"USDT_XMR"},"127":{"id":127,"baseID":214,"quoteID":243,"base":"USDT","quote":"XRP","currencyPair":"USDT_XRP"},"149":{"id":149,"baseID":214,"quoteID":267,"base":"USDT","quote":"ETH","currencyPair":"USDT_ETH"},"173":{"id":173,"baseID":214,"quoteID":283,"base":"USDT","quote":"ETC","currencyPair":"USDT_ETC"},"175":{"id":175,"baseID":214,"quoteID":284,"base":"USDT","quote":"REP","currencyPair":"USDT_REP"},"180":{"id":180,"baseID":214,"quoteID":286,"base":"USDT","quote":"ZEC","currencyPair":"USDT_ZEC"},"191":{"id":191,"baseID":214,"quoteID":292,"base":"USDT","quote":"BCH","currencyPair":"USDT_BCH"},"129":{"id":129,"baseID":240,"quoteID":17,"base":"XMR","quote":"BCN","currencyPair":"XMR_BCN"},"130":{"id":130,"baseID":240,"quoteID":22,"base":"XMR","quote":"BLK","currencyPair":"XMR_BLK"},"131":{"id":131,"baseID":240,"quoteID":29,"base":"XMR","quote":"BTCD","currencyPair":"XMR_BTCD"},"132":{"id":132,"baseID":240,"quoteID":60,"base":"XMR","quote":"DASH","currencyPair":"XMR_DASH"},"137":{"id":137,"baseID":240,"quoteID":125,"base":"XMR","quote":"LTC","currencyPair":"XMR_LTC"},"138":{"id":138,"baseID":240,"quoteID":127,"base":"XMR","quote":"MAID","currencyPair":"XMR_MAID"},"140":{"id":140,"baseID":240,"quoteID":162,"base":"XMR","quote":"NXT","currencyPair":"XMR_NXT"},"181":{"id":181,"baseID":240,"quoteID":286,"base":"XMR","quote":"ZEC","currencyPair":"XMR_ZEC"},"166":{"id":166,"baseID":267,"quoteID":278,"base":"ETH","quote":"LSK","currencyPair":"ETH_LSK"},"169":{"id":169,"baseID":267,"quoteID":281,"base":"ETH","quote":"STEEM","currencyPair":"ETH_STEEM"},"172":{"id":172,"baseID":267,"quoteID":283,"base":"ETH","quote":"ETC","currencyPair":"ETH_ETC"},"176":{"id":176,"baseID":267,"quoteID":284,"base":"ETH","quote":"REP","currencyPair":"ETH_REP"},"179":{"id":179,"baseID":267,"quoteID":286,"base":"ETH","quote":"ZEC","currencyPair":"ETH_ZEC"},"186":{"id":186,"baseID":267,"quoteID":290,"base":"ETH","quote":"GNT","currencyPair":"ETH_GNT"},"188":{"id":188,"baseID":267,"quoteID":291,"base":"ETH","quote":"GNO","currencyPair":"ETH_GNO"},"190":{"id":190,"baseID":267,"quoteID":292,"base":"ETH","quote":"BCH","currencyPair":"ETH_BCH"},"193":{"id":193,"baseID":267,"quoteID":293,"base":"ETH","quote":"ZRX","currencyPair":"ETH_ZRX"},"195":{"id":195,"baseID":267,"quoteID":294,"base":"ETH","quote":"CVC","currencyPair":"ETH_CVC"},"197":{"id":197,"baseID":267,"quoteID":295,"base":"ETH","quote":"OMG","currencyPair":"ETH_OMG"},"199":{"id":199,"baseID":267,"quoteID":296,"base":"ETH","quote":"GAS","currencyPair":"ETH_GAS"}}}"#;

#[allow(dead_code)]
lazy_static! {
  pub static ref MARKETS: MarketsStruct = get_markets();
}

#[derive(Clone, Debug)]
pub struct State {
  pub pairs: Vec<u64>,
  pub prices: HashMap<PairID, Price>,
  pub change24hs: HashMap<PairID, Change24h>,
  pub order_books: HashMap<PairID, OrderBook>
}

impl State {
  pub fn new() -> State {
    State {
      pairs: Vec::new(),
      prices: HashMap::new(),
      change24hs: HashMap::new(),
      order_books: HashMap::new()
    }
  }
}

#[derive(Clone, Debug)]
pub struct Changes<T> {
  pub changes: CircularQueue<T>,
  pub last_updates: CircularQueue<(Duration, Duration)>
}

impl<T> Changes<T> {
  pub fn new() -> Changes<T> {
    Changes::<T> {
      changes: CircularQueue::with_capacity(MAX_CAPACITY),
      last_updates: CircularQueue::with_capacity(MAX_CAPACITY)
    }
  }

  pub fn push(&mut self, item: T) -> () {
    self.changes.push(item);
    let now = Duration::seconds(Local::now().timestamp());
    let last_update;

    {
      last_update = match self.last_updates.iter().last() {
        Some(time) => time.0.clone(),
        None => now
      };
    }

    self.last_updates.push((now, now - last_update));
  }
}

pub type PairID  = u64;
pub type Volume  = f64;
pub type Sell    = Changes<Volume>;
pub type Buy     = Changes<Volume>;
pub type Percent = Changes<f64>;

pub type PriceString  = String;
pub type VolumeString = String;

#[derive(Clone, Debug)]
pub struct Price {
  pub sell: Sell,
  pub buy: Buy
}

impl Price {
  pub fn new() -> Price {
    Price {
      sell: Sell::new(),
      buy: Buy::new()
    }
  }
}

#[derive(Clone, Debug)]
pub struct Change24h {
  pub percent: Percent
}

impl Change24h {
  pub fn new() -> Change24h {
    Change24h {
      percent: Percent::new()
    }
  }
}

#[derive(Clone, Debug)]
pub struct OrderBook {
  pub sell_volume: Sell,
  pub buy_volume: Buy,
  pub depth: OrderBookSellBuyPair
}

pub type Orders = HashMap<PriceString, VolumeString>;

#[derive(Clone, Debug)]
pub struct OrderBookSellBuyPair {
  pub sell_book: Orders,
  pub buy_book: Orders
}

impl OrderBookSellBuyPair {
  pub fn new() -> OrderBookSellBuyPair {
    OrderBookSellBuyPair {
      sell_book: Orders::new(),
      buy_book: Orders::new(),
    }
  }

  pub fn get_book(&mut self, book_type: u64) -> &mut Orders {
    match book_type {
      0 => &mut self.sell_book,
      1 => &mut self.buy_book,
      _ => &mut self.sell_book
    }
  }
}

impl OrderBook {
  pub fn new() -> OrderBook {
    OrderBook {
      sell_volume: Changes::new(),
      buy_volume: Changes::new(),
      depth: OrderBookSellBuyPair::new()
    }
  }

  pub fn depth_totals(depth: &Orders) -> Volume {
    depth
      .values()
      .map(|v| v.parse::<Volume>().unwrap())
      .sum()
  }

  pub fn recalculate(self: &mut Self) {
    self.sell_volume.push(OrderBook::depth_totals(&self.depth.sell_book));
    self.buy_volume.push(OrderBook::depth_totals(&self.depth.buy_book));
  }
}

#[derive(Deserialize, Debug)]
pub struct PairDescription {
  pub id: u32,
  baseID:u32,
  quoteID: u32,
  base: String,
  quote: String,
  pub currencyPair: String
}

pub type MarketsStruct = HashMap<String, HashMap<String, PairDescription>>;

pub fn get_markets() -> MarketsStruct {
  serde_json::from_str::<MarketsStruct>(MARKETS_JSON).unwrap()
}

pub const URL:&str = "wss://api2.poloniex.com";

pub enum Channel {
  Unknown = -1,
  MarketXMR = 114,
  User = 1000,
  TrollBox,
  Ticker,
  Stats
}

macro_rules! enum_match {
  ( $number:expr, [ $( $enum_val:expr ),* ], $enum_fail_val:expr ) => {
    {
      $( if $number == $enum_val as u64 { return $enum_val } )*
      return $enum_fail_val
    }
  };
}

impl From<u64> for Channel {
  fn from(num: u64) -> Self {
    enum_match!(num, [ 
      Channel::MarketXMR,
      Channel::Ticker,
      Channel::Stats
    ], Channel::Unknown)
  }
}

pub struct MarketEvent {}
impl MarketEvent {
	pub fn handle(market_id: u64, json: &Value, state: &mut State) {
		let actions = match json[2].clone() {
			serde_json::Value::Array(arr) => arr,
			_ => vec![]
		};

		for action in actions {
			match action[0].clone() {
				serde_json::Value::String(t) => match t.as_str() {
					"i" => MarketEvent::handle_initial_sync(market_id, &action, state),
					"o" => MarketEvent::handle_order_modify(market_id, &action, state),
					"t" => MarketEvent::handle_new_trade(market_id, &action, state),
					_ => (),
				},
				_=> ()
			}
		}
	}

	fn handle_initial_sync(market_id: u64, data: &Value, state: &mut State) {
		let o = &data[1]["orderBook"];
		let (sell, buy) = (o[0].clone(), o[1].clone());
		let order_book = state.order_books
			.entry(market_id)
			.or_insert(OrderBook::new());

		order_book.depth.sell_book = serde_json
      ::from_value
      ::<HashMap<PriceString, VolumeString>>(sell)
      .unwrap();

		order_book.depth.buy_book = serde_json
      ::from_value
      ::<HashMap<PriceString, VolumeString>>(buy)
      .unwrap();

		order_book.recalculate();
	}

	fn handle_order_modify(market_id: u64, data: &Value, state: &mut State) {
		// 1 is sell or buy
		// 2 is btc price
		// 3 is volume
		let order_book = state.order_books
			.entry(market_id)
			.or_insert(OrderBook::new());

		let amount = serde_json::from_value::<String>(data[3].clone()).unwrap();
		let bid_ask_type = data[1].to_string().parse::<u64>().unwrap();
    order_book.depth.get_book(bid_ask_type)
      .insert(serde_json::from_value::<String>(data[2].clone()).unwrap(), amount);

		order_book.recalculate();
  }

	fn handle_new_trade(market_id: u64, data: &Value, state: &mut State) {
		// 2 is sell or buy
		// 3 is btc price
		// 4 is volume
		let order_book = state.order_books
			.entry(market_id)
			.or_insert(OrderBook::new());

		let amount = data[4].as_str().unwrap().to_string();
		let bid_ask_type = data[2].as_u64().unwrap();
    order_book.depth.get_book(bid_ask_type)
      .insert(data[3].as_str().unwrap().to_string(), amount);

		order_book.recalculate();
	}
}

pub struct TickerEvent {}
impl TickerEvent {
  pub fn handle(market_id: u64, json: &Value, state: &mut State) {
   let lowest_ask  = json[2][2].as_str().unwrap().parse::<Volume>().unwrap();
   let highest_bid = json[2][3].as_str().unwrap().parse::<Volume>().unwrap();
   let change      = json[2][4].as_str().unwrap().parse::<Volume>().unwrap() * (100.0 as Volume);
   let price = state
    .prices
    .entry(market_id)
    .or_insert(Price::new());

   price
    .sell
    .push(lowest_ask);

   price
    .buy
    .push(highest_bid);

   let change24h = state
    .change24hs
    .entry(market_id)
    .or_insert(Change24h::new());

   change24h
    .percent
    .push(change);
  }
}

