use starknet_crypto::Felt;

use crate::{
    group::get_fri_group,
    layer::{compute_next_layer, FriLayerComputationParams, FriLayerQuery},
};

#[test]
fn test_next_layer1() {
    let mut queries = vec![FriLayerQuery {
        index: Felt::from_dec_str("19").unwrap(),
        y_value: Felt::from_dec_str(
            "3009640132648008425771663319959959262486220333099664427328058765768842449250",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "54553547539894122403827046258314152559223416585866015792288871627619859008",
        )
        .unwrap(),
    }];
    let mut sibling_witness = vec![Felt::from_dec_str(
        "1123008634785466227765787920403783137942925653310144335875674694591276473192",
    )
    .unwrap()];
    let params = FriLayerComputationParams {
        coset_size: Felt::from(2),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2443479172752326919986485065418726216145528469562787664528210275186695390471",
        )
        .unwrap(),
    };
    let expected_next_queries = vec![FriLayerQuery {
        index: Felt::from(9),
        y_value: Felt::from_dec_str(
            "2185991974161515735589370697017442079762615282775874094325294026252936541583",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "1697286867097833219836294983908733326167501326977247172278435262719557593504",
        )
        .unwrap(),
    }];
    let expected_verify_indices = vec![Felt::from(9)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1123008634785466227765787920403783137942925653310144335875674694591276473192",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3009640132648008425771663319959959262486220333099664427328058765768842449250",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_y_values, expected_verify_y_values);
    assert_eq!(verify_indices, expected_verify_indices);
}

#[test]
fn test_next_layer2() {
    let mut queries = vec![FriLayerQuery {
        index: Felt::from(17),
        y_value: Felt::from_dec_str(
            "2759623090142790690855098103901650892631267048724128835098128315349419136695",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "108072969398534483334507045980403840714691619221758030761568305841993401905",
        )
        .unwrap(),
    }];
    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "522165234549733937327724288257767164912068272740875848443122078655609268368",
        )
        .unwrap(),
        Felt::from_dec_str(
            "62652482029070998724738900051279388447617017224333293756545714721568375652",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2828526802429981059567341974328690206758626003655409262291038410023069024668",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(4),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "1982585344331058375026005306471531766707340420809531025829328268083147325627",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![FriLayerQuery {
        index: Felt::from(4),
        y_value: Felt::from_dec_str(
            "1077998794348284354598652343293562698049252500211550192464629080328316673698",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "2633238744713263728019330547123200367032740465948237927517365901877698129956",
        )
        .unwrap(),
    }];

    let expected_verify_indices = vec![Felt::from(4)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "522165234549733937327724288257767164912068272740875848443122078655609268368",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2759623090142790690855098103901650892631267048724128835098128315349419136695",
        )
        .unwrap(),
        Felt::from_dec_str(
            "62652482029070998724738900051279388447617017224333293756545714721568375652",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2828526802429981059567341974328690206758626003655409262291038410023069024668",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_y_values, expected_verify_y_values);
    assert_eq!(verify_indices, expected_verify_indices);
}

#[test]
fn test_next_layer3() {
    let mut queries = vec![FriLayerQuery {
        index: Felt::from(22),
        y_value: Felt::from_dec_str(
            "1231175216455939321078564173742303628142533875402290207354372253959167512307",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "3549719361548528048410263413791561966844406610277484802193762821339412459253",
        )
        .unwrap(),
    }];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "1195728970024453214596842786271366866852813595571023478251747255080180444356",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3555912416221887452080706698058141424221979180551367818897445190733400868353",
        )
        .unwrap(),
        Felt::from_dec_str(
            "34862776383371782462703678371984446188640175125871204165516625259686550614",
        )
        .unwrap(),
        Felt::from_dec_str(
            "696195716834231608288788009791330705738847261355229689955205457557899136571",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2406075573647619823270675912178389603227671840135528066638010667847181325236",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1443348866565840245319023638700994949540305260981210463118495875007167932493",
        )
        .unwrap(),
        Felt::from_dec_str(
            "534188447380076871805292320071703806717768188674590858686911614529936239219",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(8),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2996681160573045082893848544757424160476352714075260471151952365319051355220",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![FriLayerQuery {
        index: Felt::from(2),
        y_value: Felt::from_dec_str(
            "1177504462272530471841134041622641579704605903617132590626888326120254257690",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "1047036369304219502363557810404294786592772605083308486097044742656766540637",
        )
        .unwrap(),
    }];
    let expected_verify_indices = vec![Felt::from(2)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1195728970024453214596842786271366866852813595571023478251747255080180444356",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3555912416221887452080706698058141424221979180551367818897445190733400868353",
        )
        .unwrap(),
        Felt::from_dec_str(
            "34862776383371782462703678371984446188640175125871204165516625259686550614",
        )
        .unwrap(),
        Felt::from_dec_str(
            "696195716834231608288788009791330705738847261355229689955205457557899136571",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2406075573647619823270675912178389603227671840135528066638010667847181325236",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1443348866565840245319023638700994949540305260981210463118495875007167932493",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1231175216455939321078564173742303628142533875402290207354372253959167512307",
        )
        .unwrap(),
        Felt::from_dec_str(
            "534188447380076871805292320071703806717768188674590858686911614529936239219",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_y_values, expected_verify_y_values);
    assert_eq!(verify_indices, expected_verify_indices);
}

#[test]
fn test_next_layer4() {
    let mut queries = vec![FriLayerQuery {
        index: Felt::from(17),
        y_value: Felt::from_dec_str(
            "107634985777525198719457933516612993333665001206557081850887267601999593307",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "2917507526425378062891209641407297049970803804883101888033200410523468508223",
        )
        .unwrap(),
    }];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "1614105094820940547932979114275805368596980954610062272415295623855966399938",
        )
        .unwrap(),
        Felt::from_dec_str(
            "693587070670384874498002073212674337957886491570174639651952635034011361113",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2182054027909863057980719290814912799438309188705871708423148386048500700847",
        )
        .unwrap(),
        Felt::from_dec_str(
            "579245916215486054657055829208838661465548354405390013595741995516616171747",
        )
        .unwrap(),
        Felt::from_dec_str(
            "206723450099456388370132180591200306073574204819219299553175570756285822026",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3524319317286904222424614883191482747595752362299160824591033064099431860213",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1615763850381464616809768799515161240452636705929717067365888125718637628955",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2729371020733114790122637155341724770739308317400814921599877096616014204365",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1806835039103743571152839213406116029435349363650311405025412410058267953618",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2334207046869053906415701809206802130396448163287773354046680229797406108795",
        )
        .unwrap(),
        Felt::from_dec_str(
            "846585632776363760500914046737920171446148160923054894684140117835211041704",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3583160895940362754785449698054871358616520492881184046653896695508292242503",
        )
        .unwrap(),
        Felt::from_dec_str(
            "594594828186152981815670652237875899521490193148447582824261477355572427969",
        )
        .unwrap(),
        Felt::from_dec_str(
            "382626503505367944553110895981418673385282307671448132202545565018242767288",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3476983551724853290554915215874670223757771126370434051488495793846971603388",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(16),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "3246137206661909741808114726375056135951563790002136626674903353340777908146",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![FriLayerQuery {
        index: Felt::from(1),
        y_value: Felt::from_dec_str(
            "1441275254889998903858094604425484342000396355353206424417863045484990540435",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "1829818947064372678210995598394063122549682555566012270431529834421656963592",
        )
        .unwrap(),
    }];
    let expected_verify_indices = vec![Felt::from(1)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1614105094820940547932979114275805368596980954610062272415295623855966399938",
        )
        .unwrap(),
        Felt::from_dec_str(
            "107634985777525198719457933516612993333665001206557081850887267601999593307",
        )
        .unwrap(),
        Felt::from_dec_str(
            "693587070670384874498002073212674337957886491570174639651952635034011361113",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2182054027909863057980719290814912799438309188705871708423148386048500700847",
        )
        .unwrap(),
        Felt::from_dec_str(
            "579245916215486054657055829208838661465548354405390013595741995516616171747",
        )
        .unwrap(),
        Felt::from_dec_str(
            "206723450099456388370132180591200306073574204819219299553175570756285822026",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3524319317286904222424614883191482747595752362299160824591033064099431860213",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1615763850381464616809768799515161240452636705929717067365888125718637628955",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2729371020733114790122637155341724770739308317400814921599877096616014204365",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1806835039103743571152839213406116029435349363650311405025412410058267953618",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2334207046869053906415701809206802130396448163287773354046680229797406108795",
        )
        .unwrap(),
        Felt::from_dec_str(
            "846585632776363760500914046737920171446148160923054894684140117835211041704",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3583160895940362754785449698054871358616520492881184046653896695508292242503",
        )
        .unwrap(),
        Felt::from_dec_str(
            "594594828186152981815670652237875899521490193148447582824261477355572427969",
        )
        .unwrap(),
        Felt::from_dec_str(
            "382626503505367944553110895981418673385282307671448132202545565018242767288",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3476983551724853290554915215874670223757771126370434051488495793846971603388",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_y_values, expected_verify_y_values);
    assert_eq!(verify_indices, expected_verify_indices);
}

#[test]
fn test_next_layer5() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "746591562936821030835715747889843595163396575226181906015101280529866617133",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2839890473743673236434239057700432645437719700778360715637471771392219034326",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(9),
            y_value: Felt::from_dec_str(
                "432934865743208963457432082990566669321350335050040579267198873031828688172",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2944384488565703107562664215590005833661704434344191167631969776808729729163",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "2207292640602765987605417203853603264378518893482237233873998269956850458932",
        )
        .unwrap(),
        Felt::from_dec_str(
            "288897229768628159275046633516552015154393884429636644820405109361062428074",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(2),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2185465603412249558871549627667757930155426034864563482170756257080076225958",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "1960029025666243333485194840332621335802675978036586078469434126072698208300",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3009148876954521410390276225740133816385634755252667189136222755512973018079",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(4),
            y_value: Felt::from_dec_str(
                "501258974496117438650836974499861760052921694630156096214408435425138801490",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1715280836225574219633496220661969295029181757403817732538849012006942881738",
            )
            .unwrap(),
        },
    ];
    let expected_verify_indices = vec![Felt::from(0), Felt::from(4)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "746591562936821030835715747889843595163396575226181906015101280529866617133",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2207292640602765987605417203853603264378518893482237233873998269956850458932",
        )
        .unwrap(),
        Felt::from_dec_str(
            "288897229768628159275046633516552015154393884429636644820405109361062428074",
        )
        .unwrap(),
        Felt::from_dec_str(
            "432934865743208963457432082990566669321350335050040579267198873031828688172",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_y_values, expected_verify_y_values);
    assert_eq!(verify_indices, expected_verify_indices);
}

#[test]
fn test_next_layer6() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "1096892199810651185152195112986319610579001690328932435764292779877576915121",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3279544897416438008064089022127475409847685010480359106267514969650981832082",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(19),
            y_value: Felt::from_dec_str(
                "1641855356491365833554692237319212189801903876432464673866647716719043041465",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2293250085048948415112043044736231472430743506786581498262287561751356762014",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "804399642855128535722354372618913360973130781887293769086134887025287828450",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1987605673743133585231493245929792399393204021630711397835563456074854788026",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1233031324852744699980974111822491085958694791795843758111112233603894427238",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1810994839619816704905050921987428341202461226961859918732361321793970914107",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1892124182925365936167405294153806740491710255246363205322154303233181243718",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1337441648818842689794955442935277091168720593363441772344593529889075124973",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(4),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2167335350971787138507326388801205881447440912920817749862011161467677463940",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "1181712651684323748024837749719027006506698122447393260787788152426027849043",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1653115732351104701033693337462649968391138809812222376293178295401862046874",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(4),
            y_value: Felt::from_dec_str(
                "1268308829684608985195972079707177792876762551009734230680402124542318187648",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1164447905508868651636386487771626691241643134212100820768777787963076904518",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(1), Felt::from(4)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "804399642855128535722354372618913360973130781887293769086134887025287828450",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1096892199810651185152195112986319610579001690328932435764292779877576915121",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1987605673743133585231493245929792399393204021630711397835563456074854788026",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1233031324852744699980974111822491085958694791795843758111112233603894427238",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1810994839619816704905050921987428341202461226961859918732361321793970914107",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1892124182925365936167405294153806740491710255246363205322154303233181243718",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1337441648818842689794955442935277091168720593363441772344593529889075124973",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1641855356491365833554692237319212189801903876432464673866647716719043041465",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer7() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(7),
            y_value: Felt::from_dec_str(
                "3075517878492540103040166847866926446865777778640852962190520816632339596145",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "608694432915418906087773663985135750833888921733668764639429291959306418477",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(27),
            y_value: Felt::from_dec_str(
                "131983826742021719455910211379866988384811584925288184480272187873636838108",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1974198664155436145837620318999161049479840212206498623330601633289848275514",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "1381946771663536271686283445165923669245048036336685207435678635918790010415",
        )
        .unwrap(),
        Felt::from_dec_str(
            "987192345921964650774860449076993743666953560925616837171412357893248515453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1894120995282262673257341015927038862976390070699798006337306824994818751092",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1538484798885111384061490573014877696930911320706092642348929689059341036045",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2831036501176131779443820825633947566967253654306554236912113528745500638916",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2795762391236757584469537519991014507421060627124928065640245513052288926693",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2962264866957003315891979312858498756904185604349911432280048675808497143212",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1274615238507625446045782756423380204591533819608003040326255896608966494630",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2433725008144059202602710302459177343649577363035970221093985127914132732545",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3615760742796657913769142930324341926341661354106616058377178459338456365110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "876491878808193133805417604582642436856178348784148668921521578336602541775",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1681379446663085402254280534212103014447726541607694761283737523967486068323",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2765034771912257979612046850994949532603486328163588398162583894664641390299",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2828844164871944409923044279501485001881689120102902225098373010416437899881",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(8),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "384372973063416357441038011066619352380639619775069754153768649812098431519",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "3519382920442250662715268714283575071860170073120425261766286332519433463480",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "266451709395649792495102217411295552361654455242989832116307089851296053314",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "3413008593626981122050040132687328477275093156117246104307650363140459009419",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1107290807486481817999551113742590485010425323783882499076376127930136554678",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(0), Felt::from(3)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1381946771663536271686283445165923669245048036336685207435678635918790010415",
        )
        .unwrap(),
        Felt::from_dec_str(
            "987192345921964650774860449076993743666953560925616837171412357893248515453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1894120995282262673257341015927038862976390070699798006337306824994818751092",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1538484798885111384061490573014877696930911320706092642348929689059341036045",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2831036501176131779443820825633947566967253654306554236912113528745500638916",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2795762391236757584469537519991014507421060627124928065640245513052288926693",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2962264866957003315891979312858498756904185604349911432280048675808497143212",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3075517878492540103040166847866926446865777778640852962190520816632339596145",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1274615238507625446045782756423380204591533819608003040326255896608966494630",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2433725008144059202602710302459177343649577363035970221093985127914132732545",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3615760742796657913769142930324341926341661354106616058377178459338456365110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "131983826742021719455910211379866988384811584925288184480272187873636838108",
        )
        .unwrap(),
        Felt::from_dec_str(
            "876491878808193133805417604582642436856178348784148668921521578336602541775",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1681379446663085402254280534212103014447726541607694761283737523967486068323",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2765034771912257979612046850994949532603486328163588398162583894664641390299",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2828844164871944409923044279501485001881689120102902225098373010416437899881",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer8() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(14),
            y_value: Felt::from_dec_str(
                "2812191291368465400252368036784849631295311044099688251864335547264648760929",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3327960025629984902906808973965115139132894388720619540209395295175202435223",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(28),
            y_value: Felt::from_dec_str(
                "945312106572582053105339715798467543580544183072055346899673660213540546488",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1982376188084661049306441397930389303876068734739596367142474139857967005184",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "2949257002824431099820924461178954292967579447203636252898226318072740386824",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2109770268208121078753001053403331765076608697772559113071033136250869504473",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2388297558783464615628997798028262325218009201379703050956169802493581047973",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3525692657203325625152712276222448417400555150313331588381183925321722344670",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1770128582224668658932820753069082677343177339697504686745994727405384368826",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2277509151931760256130277827415848713946907170081107197539171861875808782840",
        )
        .unwrap(),
        Felt::from_dec_str(
            "553506533886672046972094815070428061413496218667260813272218476397542394126",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1325795902063165511059838253657321320005294114487993699345770044761244463890",
        )
        .unwrap(),
        Felt::from_dec_str(
            "669828035369528580542389024653781788264954760854641450329504420359621694341",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3390712226968146726780043862598840971254869615115905720403199256814995219514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1826681158268766190323480513465452190809042283119310757871839564743833787919",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1145304238778090805545988213888864245409497800124759254791897692806774757770",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1397068525986579081352897876232234437189969937826283879172500553676064940439",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2104875872195729864851606343399273464419019953918563072841629145823575557945",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1158938700019692875943299533882908302674841511080317290819608130684595336457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2365573810001212351939469441049776991978930374259229467362964834664954075464",
        )
        .unwrap(),
        Felt::from_dec_str(
            "224204551411584072281213703508765114269364171685297633020108282783402412102",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1136885745908677333683443250622936914218130194199188396198410064853784245116",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2577202183647129179972590136619634609575314391523504634933521339842505024250",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1218879808821400330218160186881430073477575542748635158557783902419507248023",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1492583771642768717317176274379006337265687706897820888128354257084402032954",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1147051782978409736537631164196473096558884903070473339936475019730892606292",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1170923665353223025390644385475570698017370798971415401329877009135901058934",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2714965104434656612067886463451103798146458879447249983304400313485321454858",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3029036750822909179098438143195941816494025920675957101572384069330532165665",
        )
        .unwrap(),
        Felt::from_dec_str(
            "18409245281450708426000590154856732233696909859426757887349330897284752834",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1903399152209210559959462040157108438124673941817349364218680564081568770298",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2260002643446022813142304377372681801652650136648130581924766564336949881263",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3155765214842786052334914920203360592720185878259088994232602983562348781025",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3485201017653558756685420512495162933189527567610822904102000763401993219662",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(16),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "52871414864247307776012409456547680662415039421837737271534775961443350705",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "2731422975982182882081741213504406076248280179242291132508067217003010322490",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3183581941728587291517385359778659637268454365590489450398239584747532037461",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "3228016798076912219844600739928191932167567203345640315464632263570697784005",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "434920846937543922179937423316410468354652849741107249574852471388339983020",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(0), Felt::from(1)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "2949257002824431099820924461178954292967579447203636252898226318072740386824",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2109770268208121078753001053403331765076608697772559113071033136250869504473",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2388297558783464615628997798028262325218009201379703050956169802493581047973",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3525692657203325625152712276222448417400555150313331588381183925321722344670",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1770128582224668658932820753069082677343177339697504686745994727405384368826",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2277509151931760256130277827415848713946907170081107197539171861875808782840",
        )
        .unwrap(),
        Felt::from_dec_str(
            "553506533886672046972094815070428061413496218667260813272218476397542394126",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1325795902063165511059838253657321320005294114487993699345770044761244463890",
        )
        .unwrap(),
        Felt::from_dec_str(
            "669828035369528580542389024653781788264954760854641450329504420359621694341",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3390712226968146726780043862598840971254869615115905720403199256814995219514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1826681158268766190323480513465452190809042283119310757871839564743833787919",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1145304238778090805545988213888864245409497800124759254791897692806774757770",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1397068525986579081352897876232234437189969937826283879172500553676064940439",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2104875872195729864851606343399273464419019953918563072841629145823575557945",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2812191291368465400252368036784849631295311044099688251864335547264648760929",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1158938700019692875943299533882908302674841511080317290819608130684595336457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2365573810001212351939469441049776991978930374259229467362964834664954075464",
        )
        .unwrap(),
        Felt::from_dec_str(
            "224204551411584072281213703508765114269364171685297633020108282783402412102",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1136885745908677333683443250622936914218130194199188396198410064853784245116",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2577202183647129179972590136619634609575314391523504634933521339842505024250",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1218879808821400330218160186881430073477575542748635158557783902419507248023",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1492583771642768717317176274379006337265687706897820888128354257084402032954",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1147051782978409736537631164196473096558884903070473339936475019730892606292",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1170923665353223025390644385475570698017370798971415401329877009135901058934",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2714965104434656612067886463451103798146458879447249983304400313485321454858",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3029036750822909179098438143195941816494025920675957101572384069330532165665",
        )
        .unwrap(),
        Felt::from_dec_str(
            "18409245281450708426000590154856732233696909859426757887349330897284752834",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1903399152209210559959462040157108438124673941817349364218680564081568770298",
        )
        .unwrap(),
        Felt::from_dec_str(
            "945312106572582053105339715798467543580544183072055346899673660213540546488",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2260002643446022813142304377372681801652650136648130581924766564336949881263",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3155765214842786052334914920203360592720185878259088994232602983562348781025",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3485201017653558756685420512495162933189527567610822904102000763401993219662",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer9() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(10),
            y_value: Felt::from_dec_str(
                "3301979551015115642776773522833072926462865561144560488920209857718769165321",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2447768544415408676335928447643487449650900913681401829424717259768986472741",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(28),
            y_value: Felt::from_dec_str(
                "260750552966409642499431036714653525048800120747768231293468725416260188565",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3563529224071196680374798485546840836554333985312746699506733128078602078641",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(29),
            y_value: Felt::from_dec_str(
                "1823311453035327473280509270574026429844738895506436406781130453908277513530",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "54973564594934533322524297548229269068773230018850000466358928057269941840",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![Felt::from_dec_str(
        "3176532942878567276526900338617699157909586755721347090315239907293123933092",
    )
    .unwrap()];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(2),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "581747590720875793993157628416105910892274340828581484518962213541176928984",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "3275133527488003020272254384265923882734156428348855484827341126286781354142",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2615309274026883345480537437370039939583535394834355181916645336586036374461",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(14),
            y_value: Felt::from_dec_str(
                "2611221225089844146967129398570862707600738146407577663333768748170096584339",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1675506256769046824455009524724840487217963849380458734340688061164470257879",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(5), Felt::from(14)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "3301979551015115642776773522833072926462865561144560488920209857718769165321",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3176532942878567276526900338617699157909586755721347090315239907293123933092",
        )
        .unwrap(),
        Felt::from_dec_str(
            "260750552966409642499431036714653525048800120747768231293468725416260188565",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1823311453035327473280509270574026429844738895506436406781130453908277513530",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer10() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "2302109513041081253191847726874632617695812467358434547038386746651006842380",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2637474160357003981779699526943464856611941641202539833636026322990388707333",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(13),
            y_value: Felt::from_dec_str(
                "2421144434847175762056784279666434264733454757381019920618274713336842008821",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3087662183041901605559596075538966726399103482477428404944198306175727302850",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(22),
            y_value: Felt::from_dec_str(
                "1359842048097095741378484164247564983223481090058269548624795715692521460139",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "185282605276538732396669144005696787636433561792146384397375513028006583411",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "3354395884762193735774972945190699549697221284699421903066644213798392549402",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2286699216817078029209012960176095129558820839353213366512757899588073849784",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1807058458070289484044641026995619421272336800548938509837361700372957815942",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2613264906789519132611022617468068727030402878438205130898569384174203487539",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1731512844646762096812644150851970912816865225259598591671476871772161992367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2809244789545431804862877174186922907097032744568338981087812219597365747358",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3584511590009863696665267819675356849136392492253578574053929941069778312453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2319520140453873363680878801076253902750880806504432226644283646473284304066",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3403040452882679612874283723507253802829979450632690322212356915241676774155",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(4),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "842716418594610507165719701550798710728706432825705961415071015594374037783",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "738283733844654320656593304955937814018241368923162576526604344171964798523",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "473662685190715868134016058090152279944149415440903111962958772704259767472",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "969134386771919091675706269957962126061036556734351502037905523368628343664",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2176451726708652476016850844470876509775490912344006510026083913284976867308",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "2830771694438643841398564545385741940324054273824824277829790639217160649922",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2650304629722060934084881169738268161214211940849246175729038797083499024587",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(1), Felt::from(3), Felt::from(5)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "3354395884762193735774972945190699549697221284699421903066644213798392549402",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2302109513041081253191847726874632617695812467358434547038386746651006842380",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2286699216817078029209012960176095129558820839353213366512757899588073849784",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1807058458070289484044641026995619421272336800548938509837361700372957815942",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2613264906789519132611022617468068727030402878438205130898569384174203487539",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2421144434847175762056784279666434264733454757381019920618274713336842008821",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1731512844646762096812644150851970912816865225259598591671476871772161992367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2809244789545431804862877174186922907097032744568338981087812219597365747358",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3584511590009863696665267819675356849136392492253578574053929941069778312453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2319520140453873363680878801076253902750880806504432226644283646473284304066",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1359842048097095741378484164247564983223481090058269548624795715692521460139",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3403040452882679612874283723507253802829979450632690322212356915241676774155",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer11() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(13),
            y_value: Felt::from_dec_str(
                "2012970895874678511212757784953971039384348922302933461566125878983543084374",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2973904717437488251876439033512297970752041300850017169618797599259028370719",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(27),
            y_value: Felt::from_dec_str(
                "931874247810412416165687814293757020658355126150232036748630683953363661570",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "584004069736012205202772085193423872545377990373199751488130188652560879737",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(29),
            y_value: Felt::from_dec_str(
                "1239648551389298716869809742099762563284007154271743710729395207873328754095",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "325989084577145299562436609653878858069349249438264163435312273435225996502",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "3132112378686662059596343294646770786600609238470988213583095363604854334582",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2461092923487112471463153426310804127458685592080740231075091398736912158061",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2998509969357752287758672740593580020870026463294812456319673321073443088049",
        )
        .unwrap(),
        Felt::from_dec_str(
            "555035695605537330949170393875380217638990235130626619093741676287189098559",
        )
        .unwrap(),
        Felt::from_dec_str(
            "72018361044517802373614267173646257173391950275613610500963681187632366491",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1356567855831745726697989920693118254887070010792127324660019977162220321572",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3428903427156557890470369121770217952589955280129158495181620864445963618926",
        )
        .unwrap(),
        Felt::from_dec_str(
            "393083682526902362021532263518007213667960307630114380176309035415220154175",
        )
        .unwrap(),
        Felt::from_dec_str(
            "228421633383768634897088966132917335392548875057781107304223464921146078457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "924492636261377530436326039939616684788498986913092833064848468110206034446",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3215569544372428665026066156374535625549998775193271484978178438432055593239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "89314992381896491523088980471634454610386898184975529899102140430198625686",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3187985426015104063712461074067156374723555157039404691369397066532053540037",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(8),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "1891358614242486301056439669334692072313573517224693441784224259774268368657",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "3517289172167603711982011298083512014029690033243282325518726512433806986800",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1116757482133226363159883558057382197314289333955735205211445722872102609428",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "2418454055988269773691328349827589783726829031083650466961892768926964470890",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3351334608504925878396027162037002984645156260554050897526626427695988305073",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(1), Felt::from(3)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "3132112378686662059596343294646770786600609238470988213583095363604854334582",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2461092923487112471463153426310804127458685592080740231075091398736912158061",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2998509969357752287758672740593580020870026463294812456319673321073443088049",
        )
        .unwrap(),
        Felt::from_dec_str(
            "555035695605537330949170393875380217638990235130626619093741676287189098559",
        )
        .unwrap(),
        Felt::from_dec_str(
            "72018361044517802373614267173646257173391950275613610500963681187632366491",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2012970895874678511212757784953971039384348922302933461566125878983543084374",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1356567855831745726697989920693118254887070010792127324660019977162220321572",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3428903427156557890470369121770217952589955280129158495181620864445963618926",
        )
        .unwrap(),
        Felt::from_dec_str(
            "393083682526902362021532263518007213667960307630114380176309035415220154175",
        )
        .unwrap(),
        Felt::from_dec_str(
            "228421633383768634897088966132917335392548875057781107304223464921146078457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "924492636261377530436326039939616684788498986913092833064848468110206034446",
        )
        .unwrap(),
        Felt::from_dec_str(
            "931874247810412416165687814293757020658355126150232036748630683953363661570",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3215569544372428665026066156374535625549998775193271484978178438432055593239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1239648551389298716869809742099762563284007154271743710729395207873328754095",
        )
        .unwrap(),
        Felt::from_dec_str(
            "89314992381896491523088980471634454610386898184975529899102140430198625686",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3187985426015104063712461074067156374723555157039404691369397066532053540037",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer12() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "2526228053058749206268429435387221189473358598063225073128406154185646521246",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1904544901132915451045519411426603084952282873219697946725409068121461435004",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(10),
            y_value: Felt::from_dec_str(
                "56624383172680154199608666479407795256819238702017594377613749554501152230",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "452303108377316774348461853687416241963542162729446056484774660062242979361",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(24),
            y_value: Felt::from_dec_str(
                "1280082646894980092283175931172370782094547118060861067319301021219059720426",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1283983430867922805177012513424768198429485865021963916551986827081913277268",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "1207893144587700365996195575736463898273419338985626305569288268445097779644",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2244256048781475345139004549522339295154791205888023180969037471373425748676",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2069264725168743411785330201111662550546590368977254922367543245881704417747",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2864977467356832391348646806785103978401083442575260061290007107264804853079",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1325712426074588175917681992103732871167152828323026211822367996559374693557",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2745372160223581791751573454111775902550163309272572637725014391432686289207",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1320182822597119196092123925679279513597289598101660045113845839455396381186",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1828837204612824183172629382430137641454111146716139406496680083213113449672",
        )
        .unwrap(),
        Felt::from_dec_str(
            "292771243849180566793969674638485939456931783058705700530152077256182516555",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1853053340162949476690287745899899731365283998192199761639837549720376161302",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1574074654377970680494400284298138511458092739287721401853756095832679820400",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2335171727126212848311914093154862343081383022742695905218809639842417219817",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2863649483928914851504502227815214384723946227670079391139882735736574369209",
        )
        .unwrap(),
        Felt::from_dec_str(
            "571166807018491507338190454242538460726521500111329536305196032242467546663",
        )
        .unwrap(),
        Felt::from_dec_str(
            "279676987357461165576086433526354394307231660381247697037076852152283263764",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1820353294888025422326570312212231640999665549085940467004134021672860083252",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3576296452710656809610107747368507751192095749903159933998981729702492446131",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2977633414996464546697934803671958245149887856183204553980665750247071075567",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2931793682328107871478505893552517312756205124678921541595857767372940804931",
        )
        .unwrap(),
        Felt::from_dec_str(
            "464386720247382905224805039346212896127582735570572085216056963406710498311",
        )
        .unwrap(),
        Felt::from_dec_str(
            "320998635952014724145801465653830965824810643722569630652595245633304892795",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1243869209783079566950644774574351067214583308693372604207122773753028835268",
        )
        .unwrap(),
        Felt::from_dec_str(
            "795578854620635938716799656591963466893104246779506304831287216577244839240",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1440443221627670879258141142737734281315365036204707421342557494403242698702",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3165759076703708074008026693496870920949483211566514118299181265520560275163",
        )
        .unwrap(),
        Felt::from_dec_str(
            "847013164052415708297030782851092001846349937526001161676656062025695141455",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2724562613651009432547305409080600001190207517487511534234627332461422211487",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2777023644004316843919492669885675696223519793380600665738536014905002789857",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2553559058682947159814270185638032697395233101874392167746095003347085031030",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(16),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "3595973390675465639794922899086636220512672666972909183802502305051197462034",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "1088705866760295650273261822438240192516309122056858821372288964395003420702",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "616315024773588042326990745933816118347080550260649970905916351058572263364",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "2860448070944891293646997797340290664354871085123970576706463496050069457334",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3002187763892543171370332037161253987276026665070946729067175705077299757117",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(0), Felt::from(1)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1207893144587700365996195575736463898273419338985626305569288268445097779644",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2244256048781475345139004549522339295154791205888023180969037471373425748676",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2069264725168743411785330201111662550546590368977254922367543245881704417747",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2864977467356832391348646806785103978401083442575260061290007107264804853079",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1325712426074588175917681992103732871167152828323026211822367996559374693557",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2526228053058749206268429435387221189473358598063225073128406154185646521246",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2745372160223581791751573454111775902550163309272572637725014391432686289207",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1320182822597119196092123925679279513597289598101660045113845839455396381186",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1828837204612824183172629382430137641454111146716139406496680083213113449672",
        )
        .unwrap(),
        Felt::from_dec_str(
            "292771243849180566793969674638485939456931783058705700530152077256182516555",
        )
        .unwrap(),
        Felt::from_dec_str(
            "56624383172680154199608666479407795256819238702017594377613749554501152230",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1853053340162949476690287745899899731365283998192199761639837549720376161302",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1574074654377970680494400284298138511458092739287721401853756095832679820400",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2335171727126212848311914093154862343081383022742695905218809639842417219817",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2863649483928914851504502227815214384723946227670079391139882735736574369209",
        )
        .unwrap(),
        Felt::from_dec_str(
            "571166807018491507338190454242538460726521500111329536305196032242467546663",
        )
        .unwrap(),
        Felt::from_dec_str(
            "279676987357461165576086433526354394307231660381247697037076852152283263764",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1820353294888025422326570312212231640999665549085940467004134021672860083252",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3576296452710656809610107747368507751192095749903159933998981729702492446131",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2977633414996464546697934803671958245149887856183204553980665750247071075567",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2931793682328107871478505893552517312756205124678921541595857767372940804931",
        )
        .unwrap(),
        Felt::from_dec_str(
            "464386720247382905224805039346212896127582735570572085216056963406710498311",
        )
        .unwrap(),
        Felt::from_dec_str(
            "320998635952014724145801465653830965824810643722569630652595245633304892795",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1243869209783079566950644774574351067214583308693372604207122773753028835268",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1280082646894980092283175931172370782094547118060861067319301021219059720426",
        )
        .unwrap(),
        Felt::from_dec_str(
            "795578854620635938716799656591963466893104246779506304831287216577244839240",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1440443221627670879258141142737734281315365036204707421342557494403242698702",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3165759076703708074008026693496870920949483211566514118299181265520560275163",
        )
        .unwrap(),
        Felt::from_dec_str(
            "847013164052415708297030782851092001846349937526001161676656062025695141455",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2724562613651009432547305409080600001190207517487511534234627332461422211487",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2777023644004316843919492669885675696223519793380600665738536014905002789857",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2553559058682947159814270185638032697395233101874392167746095003347085031030",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer13() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(4),
            y_value: Felt::from_dec_str(
                "1627753637108195107008717260643443556440896293952766167989244332362025128955",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "165072171847702776025005442077416581427717054356960489404605508780869429109",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(6),
            y_value: Felt::from_dec_str(
                "1907069019505532596977567890050857167624794813827113677169100094032312679860",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2838405280269567641797768141989974821924250514269469085547129106678089679276",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(17),
            y_value: Felt::from_dec_str(
                "2667758142751156730750479133927958256743897982548140807600266277386132227210",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3170021172787255616470538597751371781796787498724713449478272545829880462903",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(19),
            y_value: Felt::from_dec_str(
                "3054802752625116251242176575121855863384552134692463894146326808649779880106",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "538689680560493478426594143215819266234245765733410283169873364502317472115",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "2050030406567804179442000347699156712126697950069244167305023529960957405037",
        )
        .unwrap(),
        Felt::from_dec_str(
            "725479814115011831742735395574739712735192127192219676877319469925485638090",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2004541427256438610478116272570683095787443576788754802866420921854549452311",
        )
        .unwrap(),
        Felt::from_dec_str(
            "724963531516986282530869066191617872380337840104897443636827319336363047197",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(2),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2421848930253683544491204126387682224910511646695020603828594542121930160761",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(2),
            y_value: Felt::from_dec_str(
                "660153636442127173118975536965070819250293153948896357172872743794635105879",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2960686226227640912988100104315064357312831954878333460443552280409792680793",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "2583053251813978881353554001887300235615403361797918894623762263894051653781",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "657816562438490300709222678780005748310275260453263239529539775726079339688",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(8),
            y_value: Felt::from_dec_str(
                "1527581375610259013472640819070111909856582743637749080241681873891116651023",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "826108911032960435877720672246591828237630390247901639819313714416153942305",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(9),
            y_value: Felt::from_dec_str(
                "1314190719421472944202120326036610156945162999618119049307656001213352574130",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2792393877633170777819602110848478277385476825083695060153778341719718078176",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(2), Felt::from(3), Felt::from(8), Felt::from(9)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1627753637108195107008717260643443556440896293952766167989244332362025128955",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2050030406567804179442000347699156712126697950069244167305023529960957405037",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1907069019505532596977567890050857167624794813827113677169100094032312679860",
        )
        .unwrap(),
        Felt::from_dec_str(
            "725479814115011831742735395574739712735192127192219676877319469925485638090",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2004541427256438610478116272570683095787443576788754802866420921854549452311",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2667758142751156730750479133927958256743897982548140807600266277386132227210",
        )
        .unwrap(),
        Felt::from_dec_str(
            "724963531516986282530869066191617872380337840104897443636827319336363047197",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3054802752625116251242176575121855863384552134692463894146326808649779880106",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer14() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(2),
            y_value: Felt::from_dec_str(
                "182467674097091120329927235354242737842583201410388016740324890375633361201",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3491761957940661772609725505931396483010302121714589766865821416607388552712",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(25),
            y_value: Felt::from_dec_str(
                "2616059464280392748910374974435988073072322805525536233200957037367537266460",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "684891698656108367482980392386620006607736687746727332990606560885224142016",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(27),
            y_value: Felt::from_dec_str(
                "3135591753368679100635971158548683977259260369951862071333071709354788461451",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2829608587555461287847580972434362343728163236965816816408509779854673967295",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(31),
            y_value: Felt::from_dec_str(
                "1881803943157720418027631826401286341924988674799418842326383947191741229239",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2061424207875745497330105211998005177924219095698929972788852026216611005243",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "2950872026957754570520920887716695721163885794330469237821164944724213529559",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3423399715507542415471039424968021876507440780439723595312075313832351840846",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2639813652387298633797735255709274903535060839700134996978742006377284413899",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1866933437322184733125022551051704017619697979136971899039533691287199399223",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1087261690083259210579547894957300747894789483945777051521564331303669475417",
        )
        .unwrap(),
        Felt::from_dec_str(
            "436336765556821905445300240111094908104143701663353530995130936976271217967",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2644311112046094225531959222370965947710384802579881971227807342025597482888",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1909838974457498375031773317733557605709575892255791166978664386630300826462",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(4),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2890900194451389976335809017939659452822058660721403378521723434210041554982",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "513745185270294796896505867131039914974805394897382564034555988323441608939",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1753073989277535228300597420267756302718024776814145631761631058811620119022",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(6),
            y_value: Felt::from_dec_str(
                "283244066499690934004140803318372542853976737301814474763000225147488436044",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1256084241920925162306077224918612704241015140814843222329579692894372909657",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(7),
            y_value: Felt::from_dec_str(
                "1478347218723203537312908341948591806836150726532123973049689074479535567310",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2362418546745206051391245558176457401382092074516753477643512363241499110824",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(0), Felt::from(6), Felt::from(7)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "2950872026957754570520920887716695721163885794330469237821164944724213529559",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3423399715507542415471039424968021876507440780439723595312075313832351840846",
        )
        .unwrap(),
        Felt::from_dec_str(
            "182467674097091120329927235354242737842583201410388016740324890375633361201",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2639813652387298633797735255709274903535060839700134996978742006377284413899",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1866933437322184733125022551051704017619697979136971899039533691287199399223",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2616059464280392748910374974435988073072322805525536233200957037367537266460",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1087261690083259210579547894957300747894789483945777051521564331303669475417",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3135591753368679100635971158548683977259260369951862071333071709354788461451",
        )
        .unwrap(),
        Felt::from_dec_str(
            "436336765556821905445300240111094908104143701663353530995130936976271217967",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2644311112046094225531959222370965947710384802579881971227807342025597482888",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1909838974457498375031773317733557605709575892255791166978664386630300826462",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1881803943157720418027631826401286341924988674799418842326383947191741229239",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer15() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "2126554718777799703033176385657191458683095484702936419714937401273860792353",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1498130324693628615611301376349242190235092933171848743274310007433732048452",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "2623153453096109592554970794864647942540496513967038379132519508620543736052",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2473811350240372293912485302342835893407435625813517542082770273218894953613",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(12),
            y_value: Felt::from_dec_str(
                "476298806093907467850716043392398955442830603875308728637021317685049559197",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1481295585313502890247351662356421852619129386517592950611879579091145907681",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(18),
            y_value: Felt::from_dec_str(
                "827426677652387725852612551358849344643252701184675574276305695934461070547",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3157568619914630228245585045460462037899356804058727670743639396696895096810",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "568564974606148495952068697014394313795111623433425386734413188543473055716",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1366545704922978306211848519149545128599600491849509699095648209057270124351",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3369664267451368476598912507893714750893169206881535467940070393280111122875",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2280690314598724729426820296417461067959633041888159626217181865005729013915",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1750406421129806494718048253683602965979894789354393453635402726451304469664",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2762059256797157000850865553676819078171564555367687766234376453546016309128",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1218810442314639785763927242813711233967407489681286827599146696588183167351",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3326413784838410331909308251183472448929379765135431872331756106367645912341",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1497242645745261449252794444194366044893668463373511650329130666278071978545",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3579089221531285967409202767297183219472499898936595220293050125486208286184",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3248438492701918352940278480328128482078022776384561858577313420268927328253",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1829392959076920522810136108754859101764285908550319866368605180895886400941",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1381649411086887915153209392042891444342882236335347987981943536021221116597",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1150216390794862013836421555089501298848998425400784049969031535552014025785",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1610452974374564984104668996356443832855149971648247845305553807032081001945",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2666891792376495085372824878310585447372725714703557465061678456052996142622",
        )
        .unwrap(),
        Felt::from_dec_str(
            "89033365488533857651117114777253473905400110863108967855831738586614375331",
        )
        .unwrap(),
        Felt::from_dec_str(
            "839735918399761087418547466208562598085415116825419145221721101951385977007",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3138433960204460263441142084968507058529045412551667347395974042720099964096",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3100876726267232844173520141766067628776170322055721109835889787928561738493",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(8),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2667333039062326164983120557556866577383303469424551586689009383516583846698",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "1298128010820835131249075641691464534391873438592600861436220192788480487820",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "563971957663528375029493660386013764689221643789293540378813704450327788085",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "2490084159237398684739244273076602421488849852729557450096588072753038507064",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3054530831002602838667829122709056340933885571542303159594278351685544232396",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(2),
            y_value: Felt::from_dec_str(
                "2291510120351370467822435604782665914711862794940021614151971912092308989788",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2933968264282979779366267770342785547048665384117568818848464375193300665318",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(0), Felt::from(1), Felt::from(2)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "568564974606148495952068697014394313795111623433425386734413188543473055716",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1366545704922978306211848519149545128599600491849509699095648209057270124351",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3369664267451368476598912507893714750893169206881535467940070393280111122875",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2126554718777799703033176385657191458683095484702936419714937401273860792353",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2280690314598724729426820296417461067959633041888159626217181865005729013915",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2623153453096109592554970794864647942540496513967038379132519508620543736052",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1750406421129806494718048253683602965979894789354393453635402726451304469664",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2762059256797157000850865553676819078171564555367687766234376453546016309128",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1218810442314639785763927242813711233967407489681286827599146696588183167351",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3326413784838410331909308251183472448929379765135431872331756106367645912341",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1497242645745261449252794444194366044893668463373511650329130666278071978545",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3579089221531285967409202767297183219472499898936595220293050125486208286184",
        )
        .unwrap(),
        Felt::from_dec_str(
            "476298806093907467850716043392398955442830603875308728637021317685049559197",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3248438492701918352940278480328128482078022776384561858577313420268927328253",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1829392959076920522810136108754859101764285908550319866368605180895886400941",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1381649411086887915153209392042891444342882236335347987981943536021221116597",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1150216390794862013836421555089501298848998425400784049969031535552014025785",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1610452974374564984104668996356443832855149971648247845305553807032081001945",
        )
        .unwrap(),
        Felt::from_dec_str(
            "827426677652387725852612551358849344643252701184675574276305695934461070547",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2666891792376495085372824878310585447372725714703557465061678456052996142622",
        )
        .unwrap(),
        Felt::from_dec_str(
            "89033365488533857651117114777253473905400110863108967855831738586614375331",
        )
        .unwrap(),
        Felt::from_dec_str(
            "839735918399761087418547466208562598085415116825419145221721101951385977007",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3138433960204460263441142084968507058529045412551667347395974042720099964096",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3100876726267232844173520141766067628776170322055721109835889787928561738493",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}

#[test]
fn test_next_layer16() {
    let mut queries = vec![
        FriLayerQuery {
            index: Felt::from(7),
            y_value: Felt::from_dec_str(
                "2970618271957846642342346764466826063381003093310142263411672089332794267936",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "696367657482296552349796637489145592059647766092429860960056113503758236140",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(15),
            y_value: Felt::from_dec_str(
                "3617920250969127201709777488707208906453088850534187183520682603094272928673",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2310133886796103194691263229141735996023430718973315327189200842370523874475",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(20),
            y_value: Felt::from_dec_str(
                "2813510573038804010549182461227254081026726929362216457735799917295807557921",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3136912033480025440798126631316085964287892167044727093790915521436523583198",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(27),
            y_value: Felt::from_dec_str(
                "2277310758645048528057712942822918302585095765258964079440830614234208634453",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1870747205314942372902696401365132965517762966704042444182117300433681492666",
            )
            .unwrap(),
        },
    ];

    let mut sibling_witness = vec![
        Felt::from_dec_str(
            "180909023427284908087068954735501773245381309706885307634112035720217679110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3516180717011553731559375883606897325832921300415665629181132188828019272220",
        )
        .unwrap(),
        Felt::from_dec_str(
            "595855061896376529488642887156753604023810348347280472583907714516066828172",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3191235363083292632152220281591154669252475323788852857089686800728410534498",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1813822547938332577999867038562586426023262706228694365068812860876915662240",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1680128694077508680748055866477369978665203502934465079629899601611040128864",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2254538912986936078053356255795224792937407530312557714459538221668304707673",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1146541571666156127290331652094519929859911664578484739484350628703870274239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2676242402429982706240946875896463048455086361143114665112503727666130770278",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1535606142568082504153083317992489160493054337756138059888305406164806817367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2327833743689290796289689852734820784384652608694315731354512839605195535594",
        )
        .unwrap(),
        Felt::from_dec_str(
            "704019990774920412035974507851267537536910401523746675504830418223883704486",
        )
        .unwrap(),
        Felt::from_dec_str(
            "819141808945878073660426874084155050645640426465236632642039400376831935514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "916452161521954486046652431769043932403407207121236049985828805160129320470",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2783827343576601773044240366051443811775748296762833339085024659475074755650",
        )
        .unwrap(),
        Felt::from_dec_str(
            "711792960833666366752081806111336906962181709004718383889531208840309577986",
        )
        .unwrap(),
        Felt::from_dec_str(
            "380931428319285290225787627529471042566157302138735167253381618771362729861",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2856797988975711654276161369071199892749361894922820391361113510391706048304",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3482933488692241523840784131023860208574894633803191769610254610727766442033",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1264899078355765869100614221263650222437429703885704857993771751181043986946",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1400484696922187703290919818865394595993978084275297840720974996479598720354",
        )
        .unwrap(),
        Felt::from_dec_str(
            "107842594321822547920522338615937431539676850957901523579287900994294462578",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1660688899193255147418136637041572395267277578278543414089972502504493135495",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2203031443746817593668322965414946570177702172611895846085976472340779485221",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2587809142332512931520259907768113553396624721896300372775193299798500202556",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3589911342720590439499798061492779810823748230816082995862628665524054571607",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2213453230887497720575673069515446618971622145035522840843819369864164611166",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2117829586952619081545500943334082335605420332616462445739700555609069867362",
        )
        .unwrap(),
    ];

    let params = FriLayerComputationParams {
        coset_size: Felt::from(16),
        fri_group: get_fri_group(),
        eval_point: Felt::from_dec_str(
            "2131161197067646908062806238455827830521701314225315670510309581734422726318",
        )
        .unwrap(),
    };

    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "1673087752917740433456711006621617791163224131325431924340953404769449739057",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3563079260215329529465469507049655514938826357303290347407524859824127636176",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "411308955591734803942114330163581819505644029281528997154872777951395903682",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "55423528450801684231853276045414590684280858028306352565567196311744384305",
            )
            .unwrap(),
        },
    ];

    let expected_verify_indices = vec![Felt::from(0), Felt::from(1)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "180909023427284908087068954735501773245381309706885307634112035720217679110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3516180717011553731559375883606897325832921300415665629181132188828019272220",
        )
        .unwrap(),
        Felt::from_dec_str(
            "595855061896376529488642887156753604023810348347280472583907714516066828172",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3191235363083292632152220281591154669252475323788852857089686800728410534498",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1813822547938332577999867038562586426023262706228694365068812860876915662240",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1680128694077508680748055866477369978665203502934465079629899601611040128864",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2254538912986936078053356255795224792937407530312557714459538221668304707673",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2970618271957846642342346764466826063381003093310142263411672089332794267936",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1146541571666156127290331652094519929859911664578484739484350628703870274239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2676242402429982706240946875896463048455086361143114665112503727666130770278",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1535606142568082504153083317992489160493054337756138059888305406164806817367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2327833743689290796289689852734820784384652608694315731354512839605195535594",
        )
        .unwrap(),
        Felt::from_dec_str(
            "704019990774920412035974507851267537536910401523746675504830418223883704486",
        )
        .unwrap(),
        Felt::from_dec_str(
            "819141808945878073660426874084155050645640426465236632642039400376831935514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "916452161521954486046652431769043932403407207121236049985828805160129320470",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3617920250969127201709777488707208906453088850534187183520682603094272928673",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2783827343576601773044240366051443811775748296762833339085024659475074755650",
        )
        .unwrap(),
        Felt::from_dec_str(
            "711792960833666366752081806111336906962181709004718383889531208840309577986",
        )
        .unwrap(),
        Felt::from_dec_str(
            "380931428319285290225787627529471042566157302138735167253381618771362729861",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2856797988975711654276161369071199892749361894922820391361113510391706048304",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2813510573038804010549182461227254081026726929362216457735799917295807557921",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3482933488692241523840784131023860208574894633803191769610254610727766442033",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1264899078355765869100614221263650222437429703885704857993771751181043986946",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1400484696922187703290919818865394595993978084275297840720974996479598720354",
        )
        .unwrap(),
        Felt::from_dec_str(
            "107842594321822547920522338615937431539676850957901523579287900994294462578",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1660688899193255147418136637041572395267277578278543414089972502504493135495",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2203031443746817593668322965414946570177702172611895846085976472340779485221",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2277310758645048528057712942822918302585095765258964079440830614234208634453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2587809142332512931520259907768113553396624721896300372775193299798500202556",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3589911342720590439499798061492779810823748230816082995862628665524054571607",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2213453230887497720575673069515446618971622145035522840843819369864164611166",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2117829586952619081545500943334082335605420332616462445739700555609069867362",
        )
        .unwrap(),
    ];

    let (next_queries, verify_indices, verify_y_values) =
        compute_next_layer(&mut queries, &mut sibling_witness, params).unwrap();

    assert_eq!(next_queries, expected_next_queries);
    assert_eq!(verify_indices, expected_verify_indices);
    assert_eq!(verify_y_values, expected_verify_y_values);
}
