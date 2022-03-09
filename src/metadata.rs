#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 4)]
        Grandpa(grandpa::Event),
        #[codec(index = 5)]
        Balances(balances::Event),
        #[codec(index = 7)]
        Sudo(sudo::Event),
        #[codec(index = 8)]
        Ipfs(ipfs::Event),
        #[codec(index = 9)]
        Scheduler(scheduler::Event),
        #[codec(index = 10)]
        Assets(assets::Event),
        #[codec(index = 11)]
        Contracts(contracts::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetChangesTrieConfig {
                pub changes_trie_config: ::core::option::Option<
                    runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                >,
            }
            impl ::subxt::Call for SetChangesTrieConfig {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_changes_trie_config";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    FillBlock,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    Remark,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SetHeapPages,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SetCode,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SetCodeWithoutChecks,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_changes_trie_config(
                    &self,
                    changes_trie_config: ::core::option::Option<
                        runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SetChangesTrieConfig,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetChangesTrieConfig {
                        changes_trie_config,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SetStorage,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    KillStorage,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    KillPrefix,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    RemarkWithEvent,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            pub struct AccountOwned(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            impl ::subxt::StorageEntry for AccountOwned {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value =
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::node_template_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BlockHash<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::node_template_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: &::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EventTopics<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 242u8, 5u8, 42u8, 1u8, 0u8, 0u8, 0u8, 0u8, 32u8, 74u8, 169u8,
                            209u8, 1u8, 0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                            192u8, 110u8, 150u8, 166u8, 46u8, 1u8, 0u8, 0u8, 1u8, 0u8, 152u8,
                            247u8, 62u8, 93u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8, 192u8,
                            246u8, 232u8, 16u8, 163u8, 1u8, 0u8, 0u8, 1u8, 0u8, 32u8, 74u8, 169u8,
                            209u8, 1u8, 0u8, 0u8, 1u8, 0u8, 136u8, 82u8, 106u8, 116u8, 0u8, 0u8,
                            0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 60u8, 0u8, 0u8, 0u8, 80u8, 0u8, 0u8, 0u8, 80u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[96u8, 9u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            64u8, 120u8, 125u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 225u8, 245u8, 5u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            52u8, 110u8, 111u8, 100u8, 101u8, 45u8, 116u8, 101u8, 109u8, 112u8,
                            108u8, 97u8, 116u8, 101u8, 52u8, 110u8, 111u8, 100u8, 101u8, 45u8,
                            116u8, 101u8, 109u8, 112u8, 108u8, 97u8, 116u8, 101u8, 1u8, 0u8, 0u8,
                            0u8, 104u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 48u8, 223u8, 106u8,
                            203u8, 104u8, 153u8, 7u8, 96u8, 155u8, 3u8, 0u8, 0u8, 0u8, 55u8, 227u8,
                            151u8, 252u8, 124u8, 145u8, 245u8, 228u8, 1u8, 0u8, 0u8, 0u8, 64u8,
                            254u8, 58u8, 212u8, 1u8, 248u8, 149u8, 154u8, 5u8, 0u8, 0u8, 0u8,
                            210u8, 188u8, 152u8, 151u8, 238u8, 208u8, 143u8, 21u8, 3u8, 0u8, 0u8,
                            0u8, 247u8, 139u8, 39u8, 139u8, 229u8, 63u8, 69u8, 76u8, 2u8, 0u8, 0u8,
                            0u8, 221u8, 113u8, 141u8, 92u8, 197u8, 50u8, 98u8, 212u8, 1u8, 0u8,
                            0u8, 0u8, 171u8, 60u8, 5u8, 114u8, 41u8, 31u8, 235u8, 139u8, 1u8, 0u8,
                            0u8, 0u8, 237u8, 153u8, 197u8, 172u8, 178u8, 94u8, 237u8, 245u8, 3u8,
                            0u8, 0u8, 0u8, 188u8, 157u8, 137u8, 144u8, 79u8, 91u8, 146u8, 63u8,
                            1u8, 0u8, 0u8, 0u8, 55u8, 200u8, 187u8, 19u8, 80u8, 169u8, 162u8,
                            168u8, 1u8, 0u8, 0u8, 0u8, 1u8, 129u8, 105u8, 205u8, 15u8, 89u8, 5u8,
                            180u8, 1u8, 0u8, 0u8, 0u8, 104u8, 182u8, 107u8, 161u8, 34u8, 201u8,
                            63u8, 167u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[42u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn random_material(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = RandomMaterial;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, Set, DispatchError, root_mod::Event>
                {
                    let call = Set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[184u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ReportEquivocation,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ReportEquivocationUnsigned,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    NoteStalled,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = NoteStalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewAuthorities(
                pub  ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            );
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession<'a>(pub &'a ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession<'_> {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[32u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetBalance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferKeepAlive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceUnreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    Transfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SetBalance,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetBalance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ForceTransfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceTransfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    TransferKeepAlive,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    TransferAll,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ForceUnreserve,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Endowed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DustLost(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Deposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReserveRepatriated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            244u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::BasicError,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn weight_to_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_support::weights::WeightToFeeCoefficient<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            4u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SudoAs {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, Sudo, DispatchError, root_mod::Event>
                {
                    let call = Sudo {
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SudoUncheckedWeight,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SudoUncheckedWeight {
                        call: ::std::boxed::Box::new(call),
                        weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SetKey,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SudoAs,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SudoAs {
                        who,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Sudid(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KeyChanged(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SudoAsDone(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod ipfs {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CreateIpfsAsset {
                pub addr: ::std::vec::Vec<::core::primitive::u8>,
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for CreateIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "create_ipfs_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CreateIpfsAssetRaw {
                pub addr: ::std::vec::Vec<::core::primitive::u8>,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for CreateIpfsAssetRaw {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "create_ipfs_asset_raw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PinIpfsAsset {
                pub addr: ::std::vec::Vec<::core::primitive::u8>,
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for PinIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "pin_ipfs_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnpinIpfsAsset {
                pub addr: ::std::vec::Vec<::core::primitive::u8>,
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for UnpinIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "unpin_ipfs_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DeleteIpfsAsset {
                pub addr: ::std::vec::Vec<::core::primitive::u8>,
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for DeleteIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "delete_ipfs_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReadFile {
                pub addr: ::std::vec::Vec<::core::primitive::u8>,
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ReadFile {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "read_file";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SubmitIpfsIdentity {
                pub public_key: ::std::vec::Vec<::core::primitive::u8>,
                pub multiaddress:
                    ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
            }
            impl ::subxt::Call for SubmitIpfsIdentity {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "submit_ipfs_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SubmitIpfsAddResults {
                pub admin: ::subxt::sp_core::crypto::AccountId32,
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SubmitIpfsAddResults {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "submit_ipfs_add_results";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SubmitIpfsPinResults {
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SubmitIpfsPinResults {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "submit_ipfs_pin_results";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SubmitIpfsUnpinResults {
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SubmitIpfsUnpinResults {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "submit_ipfs_unpin_results";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SubmitIpfsDeleteResults {
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SubmitIpfsDeleteResults {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "submit_ipfs_delete_results";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AddOwner {
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
                pub add_acct: ::subxt::sp_core::crypto::AccountId32,
                pub ownership_layer: runtime_types::pallet_ipfs::pallet::OwnershipLayer,
            }
            impl ::subxt::Call for AddOwner {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "add_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct RemoveOwner {
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
                pub remove_acct: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for RemoveOwner {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "remove_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ChangeOwnership {
                pub cid: ::std::vec::Vec<::core::primitive::u8>,
                pub acct_to_change: ::subxt::sp_core::crypto::AccountId32,
                pub ownership_layer: runtime_types::pallet_ipfs::pallet::OwnershipLayer,
            }
            impl ::subxt::Call for ChangeOwnership {
                const PALLET: &'static str = "Ipfs";
                const FUNCTION: &'static str = "change_ownership";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn create_ipfs_asset(
                    &self,
                    addr: ::std::vec::Vec<::core::primitive::u8>,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    CreateIpfsAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = CreateIpfsAsset { addr, cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create_ipfs_asset_raw(
                    &self,
                    addr: ::std::vec::Vec<::core::primitive::u8>,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    CreateIpfsAssetRaw,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = CreateIpfsAssetRaw { addr, data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn pin_ipfs_asset(
                    &self,
                    addr: ::std::vec::Vec<::core::primitive::u8>,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    PinIpfsAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = PinIpfsAsset { addr, cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unpin_ipfs_asset(
                    &self,
                    addr: ::std::vec::Vec<::core::primitive::u8>,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    UnpinIpfsAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = UnpinIpfsAsset { addr, cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn delete_ipfs_asset(
                    &self,
                    addr: ::std::vec::Vec<::core::primitive::u8>,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    DeleteIpfsAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = DeleteIpfsAsset { addr, cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn read_file(
                    &self,
                    addr: ::std::vec::Vec<::core::primitive::u8>,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ReadFile,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReadFile { addr, cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_ipfs_identity(
                    &self,
                    public_key: ::std::vec::Vec<::core::primitive::u8>,
                    multiaddress: ::std::vec::Vec<
                        runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SubmitIpfsIdentity,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitIpfsIdentity {
                        public_key,
                        multiaddress,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_ipfs_add_results(
                    &self,
                    admin: ::subxt::sp_core::crypto::AccountId32,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SubmitIpfsAddResults,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitIpfsAddResults { admin, cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_ipfs_pin_results(
                    &self,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SubmitIpfsPinResults,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitIpfsPinResults { cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_ipfs_unpin_results(
                    &self,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SubmitIpfsUnpinResults,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitIpfsUnpinResults { cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_ipfs_delete_results(
                    &self,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    SubmitIpfsDeleteResults,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitIpfsDeleteResults { cid };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_owner(
                    &self,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                    add_acct: ::subxt::sp_core::crypto::AccountId32,
                    ownership_layer: runtime_types::pallet_ipfs::pallet::OwnershipLayer,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    AddOwner,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = AddOwner {
                        cid,
                        add_acct,
                        ownership_layer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_owner(
                    &self,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                    remove_acct: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    RemoveOwner,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = RemoveOwner { cid, remove_acct };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn change_ownership(
                    &self,
                    cid: ::std::vec::Vec<::core::primitive::u8>,
                    acct_to_change: ::subxt::sp_core::crypto::AccountId32,
                    ownership_layer: runtime_types::pallet_ipfs::pallet::OwnershipLayer,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ChangeOwnership,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ChangeOwnership {
                        cid,
                        acct_to_change,
                        ownership_layer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_ipfs::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct QueuedDataToAdd(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for QueuedDataToAdd {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "QueuedDataToAdd";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct QueuedDataToCat(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for QueuedDataToCat {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "QueuedDataToCat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PublishedIdentity(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for PublishedIdentity {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "PublishedIdentity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Created(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PriceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub ::core::option::Option<::core::primitive::u128>,
            );
            impl ::subxt::Event for PriceSet {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "PriceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transferred(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Bought(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Bought {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "Bought";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AddOwner(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for AddOwner {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "AddOwner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct RemoveOwner(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for RemoveOwner {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "RemoveOwner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ChangeOwnershipLayer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ChangeOwnershipLayer {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "ChangeOwnershipLayer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct WriteIpfsAsset(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for WriteIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "WriteIpfsAsset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReadIpfsAsset(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for ReadIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "ReadIpfsAsset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DeleteIpfsAsset(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for DeleteIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "DeleteIpfsAsset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AddPin(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for AddPin {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "AddPin";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnpinIpfsAsset(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for UnpinIpfsAsset {
                const PALLET: &'static str = "Ipfs";
                const EVENT: &'static str = "UnpinIpfsAsset";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct BootstrapNodes<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for BootstrapNodes<'_> {
                const PALLET: &'static str = "Ipfs";
                const STORAGE: &'static str = "BootstrapNodes";
                type Value = ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct DataQueue;
            impl ::subxt::StorageEntry for DataQueue {
                const PALLET: &'static str = "Ipfs";
                const STORAGE: &'static str = "DataQueue";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_ipfs::DataCommand<::subxt::sp_core::crypto::AccountId32>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct IpfsCnt;
            impl ::subxt::StorageEntry for IpfsCnt {
                const PALLET: &'static str = "Ipfs";
                const STORAGE: &'static str = "IpfsCnt";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct IpfsAsset<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for IpfsAsset<'_> {
                const PALLET: &'static str = "Ipfs";
                const STORAGE: &'static str = "IpfsAsset";
                type Value = runtime_types::pallet_ipfs::pallet::Ipfs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct IpfsAssetOwned<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for IpfsAssetOwned<'_> {
                const PALLET: &'static str = "Ipfs";
                const STORAGE: &'static str = "IpfsAssetOwned";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::std::vec::Vec<::core::primitive::u8>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn bootstrap_nodes(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                    ::subxt::BasicError,
                > {
                    let entry = BootstrapNodes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bootstrap_nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BootstrapNodes<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn data_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_ipfs::DataCommand<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = DataQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ipfs_cnt(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = IpfsCnt;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ipfs_asset(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_ipfs::pallet::Ipfs>,
                    ::subxt::BasicError,
                > {
                    let entry = IpfsAsset(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn ipfs_asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, IpfsAsset<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn ipfs_asset_owned(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = IpfsAssetOwned(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ipfs_asset_owned_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, IpfsAssetOwned<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_ipfs_owned(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[5u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod scheduler {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for Schedule {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Cancel {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ScheduleNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for ScheduleNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CancelNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for CancelNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ScheduleAfter {
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for ScheduleAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_after";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ScheduleNamedAfter {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for ScheduleNamedAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named_after";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    Schedule,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Schedule {
                        when,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    Cancel,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Cancel { when, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ScheduleNamed,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ScheduleNamed {
                        id,
                        when,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    CancelNamed,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = CancelNamed { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ScheduleAfter,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ScheduleAfter {
                        after,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named_after(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    ScheduleNamedAfter,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ScheduleNamedAfter {
                        id,
                        after,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Scheduled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Canceled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Dispatched(
                pub (::core::primitive::u32, ::core::primitive::u32),
                pub ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Agenda<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for Agenda<'_> {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Agenda";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<
                        runtime_types::pallet_scheduler::ScheduledV2<
                            runtime_types::node_template_runtime::Call,
                            ::core::primitive::u32,
                            runtime_types::node_template_runtime::OriginCaller,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Lookup<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for Lookup<'_> {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Lookup";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_scheduler::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn agenda(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<
                            runtime_types::pallet_scheduler::ScheduledV2<
                                runtime_types::node_template_runtime::Call,
                                ::core::primitive::u32,
                                runtime_types::node_template_runtime::OriginCaller,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Agenda(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn agenda_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Agenda<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn lookup(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = Lookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Lookup<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_scheduler::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn maximum_weight(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 128u8, 110u8, 135u8, 116u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_scheduled_per_block(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod assets {
        use super::root_mod;
        use super::runtime_types;
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Created(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Issued(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transferred(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Burned(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TeamChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OwnerChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Frozen(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Thawed(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetFrozen(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetFrozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetThawed(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetThawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Destroyed(pub ::core::primitive::u32);
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceCreated(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MetadataSet(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u8,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct MetadataCleared(pub ::core::primitive::u32);
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApprovedTransfer(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApprovalCancelled(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferredApproved(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for TransferredApproved {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetStatusChanged(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Asset<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset<'_> {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_assets::types::AssetDetails<
                    ::core::primitive::u64,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Account";
                type Value =
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u64, ()>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Approvals<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Approvals<'_> {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types::pallet_assets::types::Approval<
                    ::core::primitive::u64,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Metadata<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for Metadata<'_> {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Metadata";
                type Value = runtime_types::pallet_assets::types::AssetMetadata<
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn asset(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u64,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Asset(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u64, ()>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    _2: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u64,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Approvals(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approvals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Approvals<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn metadata(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Metadata(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn metadata_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Metadata<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn asset_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 193u8, 111u8, 242u8, 134u8, 35u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 198u8, 164u8, 126u8, 141u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 64u8, 122u8, 16u8, 243u8, 90u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn approval_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 64u8, 122u8, 16u8, 243u8, 90u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn string_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod contracts {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Call {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Call {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "call";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InstantiateWithCode {
                #[codec(compact)]
                pub endowment: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub code: ::std::vec::Vec<::core::primitive::u8>,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for InstantiateWithCode {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "instantiate_with_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Instantiate {
                #[codec(compact)]
                pub endowment: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub code_hash: ::subxt::sp_core::H256,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Instantiate {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "instantiate";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn call(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, Call, DispatchError, root_mod::Event>
                {
                    let call = Call {
                        dest,
                        value,
                        gas_limit,
                        data,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn instantiate_with_code(
                    &self,
                    endowment: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    InstantiateWithCode,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = InstantiateWithCode {
                        endowment,
                        gas_limit,
                        code,
                        data,
                        salt,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn instantiate(
                    &self,
                    endowment: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    code_hash: ::subxt::sp_core::H256,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    Instantiate,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Instantiate {
                        endowment,
                        gas_limit,
                        code_hash,
                        data,
                        salt,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_contracts::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Instantiated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Instantiated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Instantiated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Terminated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Terminated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Terminated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CodeStored(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for CodeStored {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeStored";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ScheduleUpdated(pub ::core::primitive::u32);
            impl ::subxt::Event for ScheduleUpdated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ScheduleUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ContractEmitted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for ContractEmitted {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractEmitted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CodeRemoved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for CodeRemoved {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PristineCode<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for PristineCode<'_> {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "PristineCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CodeStorage<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for CodeStorage<'_> {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "CodeStorage";
                type Value = runtime_types::pallet_contracts::wasm::PrefabWasmModule;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct AccountCounter;
            impl ::subxt::StorageEntry for AccountCounter {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "AccountCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ContractInfoOf<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ContractInfoOf<'_> {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "ContractInfoOf";
                type Value = runtime_types::pallet_contracts::storage::RawContractInfo<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DeletionQueue;
            impl ::subxt::StorageEntry for DeletionQueue {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "DeletionQueue";
                type Value =
                    ::std::vec::Vec<runtime_types::pallet_contracts::storage::DeletedContract>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn pristine_code(
                    &self,
                    _0: &::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = PristineCode(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pristine_code_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PristineCode<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn code_storage(
                    &self,
                    _0: &::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_contracts::wasm::PrefabWasmModule>,
                    ::subxt::BasicError,
                > {
                    let entry = CodeStorage(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn code_storage_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CodeStorage<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn account_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = AccountCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn contract_info_of(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_contracts::storage::RawContractInfo<
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ContractInfoOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn contract_info_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ContractInfoOf<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn deletion_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::pallet_contracts::storage::DeletedContract>,
                    ::subxt::BasicError,
                > {
                    let entry = DeletionQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn schedule(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::pallet_contracts::schedule::Schedule,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            4u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 128u8, 0u8,
                            0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
                            32u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8,
                            0u8, 4u8, 0u8, 2u8, 0u8, 0u8, 0u8, 164u8, 21u8, 0u8, 0u8, 240u8, 50u8,
                            0u8, 0u8, 74u8, 61u8, 0u8, 0u8, 40u8, 15u8, 0u8, 0u8, 206u8, 34u8, 0u8,
                            0u8, 244u8, 21u8, 0u8, 0u8, 24u8, 11u8, 0u8, 0u8, 82u8, 28u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 70u8, 193u8, 2u8, 0u8, 168u8, 3u8, 4u8, 0u8, 200u8,
                            15u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 192u8, 8u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 154u8, 21u8, 0u8, 0u8, 82u8, 23u8, 0u8, 0u8, 36u8, 19u8, 0u8,
                            0u8, 155u8, 104u8, 74u8, 2u8, 142u8, 18u8, 0u8, 0u8, 142u8, 18u8, 0u8,
                            0u8, 142u8, 18u8, 0u8, 0u8, 112u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 42u8, 18u8, 0u8, 0u8, 164u8, 16u8, 0u8, 0u8, 134u8,
                            16u8, 0u8, 0u8, 100u8, 15u8, 0u8, 0u8, 30u8, 15u8, 0u8, 0u8, 176u8,
                            14u8, 0u8, 0u8, 94u8, 16u8, 0u8, 0u8, 174u8, 16u8, 0u8, 0u8, 114u8,
                            16u8, 0u8, 0u8, 254u8, 16u8, 0u8, 0u8, 214u8, 16u8, 0u8, 0u8, 104u8,
                            16u8, 0u8, 0u8, 4u8, 16u8, 0u8, 0u8, 210u8, 15u8, 0u8, 0u8, 166u8,
                            39u8, 0u8, 0u8, 190u8, 30u8, 0u8, 0u8, 114u8, 41u8, 0u8, 0u8, 20u8,
                            30u8, 0u8, 0u8, 214u8, 16u8, 0u8, 0u8, 14u8, 16u8, 0u8, 0u8, 90u8,
                            15u8, 0u8, 0u8, 50u8, 15u8, 0u8, 0u8, 174u8, 16u8, 0u8, 0u8, 80u8,
                            15u8, 0u8, 0u8, 20u8, 15u8, 0u8, 0u8, 220u8, 15u8, 0u8, 0u8, 244u8,
                            37u8, 19u8, 0u8, 0u8, 0u8, 0u8, 0u8, 222u8, 49u8, 19u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 104u8, 248u8, 18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 26u8, 30u8,
                            54u8, 0u8, 0u8, 0u8, 0u8, 0u8, 190u8, 56u8, 19u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 214u8, 22u8, 19u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 62u8, 19u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 84u8, 17u8, 19u8, 0u8, 0u8, 0u8, 0u8, 0u8, 40u8,
                            26u8, 19u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 87u8, 44u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 24u8, 173u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 26u8, 83u8, 16u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 115u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            216u8, 245u8, 99u8, 0u8, 0u8, 0u8, 0u8, 0u8, 104u8, 2u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 114u8, 164u8, 32u8, 0u8, 0u8, 0u8, 0u8, 56u8, 9u8,
                            60u8, 0u8, 0u8, 0u8, 0u8, 0u8, 170u8, 105u8, 105u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 126u8, 78u8, 189u8, 7u8, 0u8, 0u8, 0u8, 0u8, 83u8, 6u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 216u8, 28u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 252u8,
                            75u8, 27u8, 6u8, 0u8, 0u8, 0u8, 0u8, 191u8, 2u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 30u8, 24u8, 6u8, 0u8, 0u8, 0u8, 0u8, 250u8, 246u8,
                            208u8, 1u8, 0u8, 0u8, 0u8, 0u8, 71u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 202u8, 235u8, 8u8, 10u8, 0u8, 0u8, 0u8, 0u8, 122u8, 97u8, 45u8,
                            31u8, 0u8, 0u8, 0u8, 0u8, 90u8, 206u8, 215u8, 9u8, 0u8, 0u8, 0u8, 0u8,
                            108u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 3u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 46u8, 243u8, 1u8, 50u8, 0u8, 0u8, 0u8, 0u8, 107u8, 2u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 215u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            177u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 94u8, 106u8, 20u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 11u8, 19u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 120u8,
                            21u8, 0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 13u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 88u8, 85u8, 17u8, 0u8, 0u8, 0u8, 0u8, 0u8, 66u8, 6u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 18u8, 70u8, 17u8, 0u8, 0u8, 0u8, 0u8, 0u8, 66u8,
                            6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 194u8, 42u8, 80u8, 9u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn contract_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            100u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn deletion_queue_depth(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[162u8, 186u8, 1u8, 0u8][..],
                    )?)
                }
                pub fn deletion_weight_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 208u8, 237u8, 144u8, 46u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    set_changes_trie_config {
                        changes_trie_config: ::core::option::Option<
                            runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                        >,
                    },
                    #[codec(index = 6)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 7)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 8)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(_0),
                #[codec(index = 2)]
                None,
            }
        }
        pub mod node_template_runtime {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 4)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 8)]
                Ipfs(runtime_types::pallet_ipfs::pallet::Call),
                #[codec(index = 9)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                #[codec(index = 11)]
                Contracts(runtime_types::pallet_contracts::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 8)]
                Ipfs(runtime_types::pallet_ipfs::pallet::Event),
                #[codec(index = 9)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Event),
                #[codec(index = 10)]
                Assets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 11)]
                Contracts(runtime_types::pallet_contracts::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_system::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
                ),
                #[codec(index = 1)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Runtime;
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    BalanceZero,
                    #[codec(index = 2)]
                    NoPermission,
                    #[codec(index = 3)]
                    Unknown,
                    #[codec(index = 4)]
                    Frozen,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    BadWitness,
                    #[codec(index = 7)]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    NoProvider,
                    #[codec(index = 9)]
                    BadMetadata,
                    #[codec(index = 10)]
                    Unapproved,
                    #[codec(index = 11)]
                    WouldDie,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Created(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    Issued(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 2)]
                    Transferred(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    Burned(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 4)]
                    TeamChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 5)]
                    OwnerChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 6)]
                    Frozen(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 7)]
                    Thawed(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 8)]
                    AssetFrozen(::core::primitive::u32),
                    #[codec(index = 9)]
                    AssetThawed(::core::primitive::u32),
                    #[codec(index = 10)]
                    Destroyed(::core::primitive::u32),
                    #[codec(index = 11)]
                    ForceCreated(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 12)]
                    MetadataSet(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u8,
                        ::core::primitive::bool,
                    ),
                    #[codec(index = 13)]
                    MetadataCleared(::core::primitive::u32),
                    #[codec(index = 14)]
                    ApprovedTransfer(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 15)]
                    ApprovalCancelled(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 16)]
                    TransferredApproved(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 17)]
                    AssetStatusChanged(::core::primitive::u32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AssetBalance<_0, _1> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub sufficient: ::core::primitive::bool,
                    pub extra: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _2,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    DustLost(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Transfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    BalanceSet(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    Deposit(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 5)]
                    Reserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 6)]
                    Unreserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 7)]
                    ReserveRepatriated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_contracts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    call {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    instantiate_with_code {
                        #[codec(compact)]
                        endowment: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    instantiate {
                        #[codec(compact)]
                        endowment: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        code_hash: ::subxt::sp_core::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidScheduleVersion,
                    #[codec(index = 1)]
                    OutOfGas,
                    #[codec(index = 2)]
                    OutputBufferTooSmall,
                    #[codec(index = 3)]
                    BelowSubsistenceThreshold,
                    #[codec(index = 4)]
                    NewContractNotFunded,
                    #[codec(index = 5)]
                    TransferFailed,
                    #[codec(index = 6)]
                    MaxCallDepthReached,
                    #[codec(index = 7)]
                    ContractNotFound,
                    #[codec(index = 8)]
                    CodeTooLarge,
                    #[codec(index = 9)]
                    CodeNotFound,
                    #[codec(index = 10)]
                    OutOfBounds,
                    #[codec(index = 11)]
                    DecodingFailed,
                    #[codec(index = 12)]
                    ContractTrapped,
                    #[codec(index = 13)]
                    ValueTooLarge,
                    #[codec(index = 14)]
                    TerminatedWhileReentrant,
                    #[codec(index = 15)]
                    InputForwarded,
                    #[codec(index = 16)]
                    RandomSubjectTooLong,
                    #[codec(index = 17)]
                    TooManyTopics,
                    #[codec(index = 18)]
                    DuplicateTopics,
                    #[codec(index = 19)]
                    NoChainExtension,
                    #[codec(index = 20)]
                    DeletionQueueFull,
                    #[codec(index = 21)]
                    StorageExhausted,
                    #[codec(index = 22)]
                    DuplicateContract,
                    #[codec(index = 23)]
                    TerminatedInConstructor,
                    #[codec(index = 24)]
                    DebugMessageInvalidUTF8,
                    #[codec(index = 25)]
                    ReentranceDenied,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Instantiated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    Terminated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 2)]
                    CodeStored(::subxt::sp_core::H256),
                    #[codec(index = 3)]
                    ScheduleUpdated(::core::primitive::u32),
                    #[codec(index = 4)]
                    ContractEmitted(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 5)]
                    CodeRemoved(::subxt::sp_core::H256),
                }
            }
            pub mod schedule {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct HostFnWeights {
                    pub caller: ::core::primitive::u64,
                    pub address: ::core::primitive::u64,
                    pub gas_left: ::core::primitive::u64,
                    pub balance: ::core::primitive::u64,
                    pub value_transferred: ::core::primitive::u64,
                    pub minimum_balance: ::core::primitive::u64,
                    pub contract_deposit: ::core::primitive::u64,
                    pub block_number: ::core::primitive::u64,
                    pub now: ::core::primitive::u64,
                    pub weight_to_fee: ::core::primitive::u64,
                    pub gas: ::core::primitive::u64,
                    pub input: ::core::primitive::u64,
                    pub input_per_byte: ::core::primitive::u64,
                    pub r#return: ::core::primitive::u64,
                    pub return_per_byte: ::core::primitive::u64,
                    pub terminate: ::core::primitive::u64,
                    pub random: ::core::primitive::u64,
                    pub deposit_event: ::core::primitive::u64,
                    pub deposit_event_per_topic: ::core::primitive::u64,
                    pub deposit_event_per_byte: ::core::primitive::u64,
                    pub debug_message: ::core::primitive::u64,
                    pub set_storage: ::core::primitive::u64,
                    pub set_storage_per_byte: ::core::primitive::u64,
                    pub clear_storage: ::core::primitive::u64,
                    pub get_storage: ::core::primitive::u64,
                    pub get_storage_per_byte: ::core::primitive::u64,
                    pub transfer: ::core::primitive::u64,
                    pub call: ::core::primitive::u64,
                    pub call_transfer_surcharge: ::core::primitive::u64,
                    pub call_per_input_byte: ::core::primitive::u64,
                    pub call_per_output_byte: ::core::primitive::u64,
                    pub instantiate: ::core::primitive::u64,
                    pub instantiate_per_input_byte: ::core::primitive::u64,
                    pub instantiate_per_output_byte: ::core::primitive::u64,
                    pub instantiate_per_salt_byte: ::core::primitive::u64,
                    pub hash_sha2_256: ::core::primitive::u64,
                    pub hash_sha2_256_per_byte: ::core::primitive::u64,
                    pub hash_keccak_256: ::core::primitive::u64,
                    pub hash_keccak_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_256: ::core::primitive::u64,
                    pub hash_blake2_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_128: ::core::primitive::u64,
                    pub hash_blake2_128_per_byte: ::core::primitive::u64,
                    pub ecdsa_recover: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct InstructionWeights {
                    pub version: ::core::primitive::u32,
                    pub i64const: ::core::primitive::u32,
                    pub i64load: ::core::primitive::u32,
                    pub i64store: ::core::primitive::u32,
                    pub select: ::core::primitive::u32,
                    pub r#if: ::core::primitive::u32,
                    pub br: ::core::primitive::u32,
                    pub br_if: ::core::primitive::u32,
                    pub br_table: ::core::primitive::u32,
                    pub br_table_per_entry: ::core::primitive::u32,
                    pub call: ::core::primitive::u32,
                    pub call_indirect: ::core::primitive::u32,
                    pub call_indirect_per_param: ::core::primitive::u32,
                    pub local_get: ::core::primitive::u32,
                    pub local_set: ::core::primitive::u32,
                    pub local_tee: ::core::primitive::u32,
                    pub global_get: ::core::primitive::u32,
                    pub global_set: ::core::primitive::u32,
                    pub memory_current: ::core::primitive::u32,
                    pub memory_grow: ::core::primitive::u32,
                    pub i64clz: ::core::primitive::u32,
                    pub i64ctz: ::core::primitive::u32,
                    pub i64popcnt: ::core::primitive::u32,
                    pub i64eqz: ::core::primitive::u32,
                    pub i64extendsi32: ::core::primitive::u32,
                    pub i64extendui32: ::core::primitive::u32,
                    pub i32wrapi64: ::core::primitive::u32,
                    pub i64eq: ::core::primitive::u32,
                    pub i64ne: ::core::primitive::u32,
                    pub i64lts: ::core::primitive::u32,
                    pub i64ltu: ::core::primitive::u32,
                    pub i64gts: ::core::primitive::u32,
                    pub i64gtu: ::core::primitive::u32,
                    pub i64les: ::core::primitive::u32,
                    pub i64leu: ::core::primitive::u32,
                    pub i64ges: ::core::primitive::u32,
                    pub i64geu: ::core::primitive::u32,
                    pub i64add: ::core::primitive::u32,
                    pub i64sub: ::core::primitive::u32,
                    pub i64mul: ::core::primitive::u32,
                    pub i64divs: ::core::primitive::u32,
                    pub i64divu: ::core::primitive::u32,
                    pub i64rems: ::core::primitive::u32,
                    pub i64remu: ::core::primitive::u32,
                    pub i64and: ::core::primitive::u32,
                    pub i64or: ::core::primitive::u32,
                    pub i64xor: ::core::primitive::u32,
                    pub i64shl: ::core::primitive::u32,
                    pub i64shrs: ::core::primitive::u32,
                    pub i64shru: ::core::primitive::u32,
                    pub i64rotl: ::core::primitive::u32,
                    pub i64rotr: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Limits {
                    pub event_topics: ::core::primitive::u32,
                    pub stack_height: ::core::primitive::u32,
                    pub globals: ::core::primitive::u32,
                    pub parameters: ::core::primitive::u32,
                    pub memory_pages: ::core::primitive::u32,
                    pub table_size: ::core::primitive::u32,
                    pub br_table_size: ::core::primitive::u32,
                    pub subject_len: ::core::primitive::u32,
                    pub call_depth: ::core::primitive::u32,
                    pub payload_len: ::core::primitive::u32,
                    pub code_len: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Schedule {
                    pub limits: runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights: runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }
            pub mod storage {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DeletedContract {
                    pub trie_id: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct RawContractInfo<_0> {
                    pub trie_id: ::std::vec::Vec<::core::primitive::u8>,
                    pub code_hash: _0,
                    pub _reserved: ::core::option::Option<()>,
                }
            }
            pub mod wasm {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct PrefabWasmModule {
                    #[codec(compact)]
                    pub instruction_weights_version: ::core::primitive::u32,
                    #[codec(compact)]
                    pub initial: ::core::primitive::u32,
                    #[codec(compact)]
                    pub maximum: ::core::primitive::u32,
                    #[codec(compact)]
                    pub refcount: ::core::primitive::u64,
                    pub _reserved: ::core::option::Option<()>,
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                    pub original_code_len: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities(
                        ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    ),
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_ipfs {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    create_ipfs_asset {
                        addr: ::std::vec::Vec<::core::primitive::u8>,
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    create_ipfs_asset_raw {
                        addr: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    pin_ipfs_asset {
                        addr: ::std::vec::Vec<::core::primitive::u8>,
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    unpin_ipfs_asset {
                        addr: ::std::vec::Vec<::core::primitive::u8>,
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    delete_ipfs_asset {
                        addr: ::std::vec::Vec<::core::primitive::u8>,
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    read_file {
                        addr: ::std::vec::Vec<::core::primitive::u8>,
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 6)]
                    submit_ipfs_identity {
                        public_key: ::std::vec::Vec<::core::primitive::u8>,
                        multiaddress:
                            ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                    },
                    #[codec(index = 7)]
                    submit_ipfs_add_results {
                        admin: ::subxt::sp_core::crypto::AccountId32,
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    submit_ipfs_pin_results {
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    submit_ipfs_unpin_results {
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 10)]
                    submit_ipfs_delete_results {
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 11)]
                    add_owner {
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                        add_acct: ::subxt::sp_core::crypto::AccountId32,
                        ownership_layer: runtime_types::pallet_ipfs::pallet::OwnershipLayer,
                    },
                    #[codec(index = 12)]
                    remove_owner {
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                        remove_acct: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 13)]
                    change_ownership {
                        cid: ::std::vec::Vec<::core::primitive::u8>,
                        acct_to_change: ::subxt::sp_core::crypto::AccountId32,
                        ownership_layer: runtime_types::pallet_ipfs::pallet::OwnershipLayer,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    IpfsCntOverflow,
                    #[codec(index = 1)]
                    ExceedMaxIpfsOwned,
                    #[codec(index = 2)]
                    BuyerIsIpfsOwner,
                    #[codec(index = 3)]
                    TransferToSelf,
                    #[codec(index = 4)]
                    IpfsNotExist,
                    #[codec(index = 5)]
                    AccNotExist,
                    #[codec(index = 6)]
                    NotIpfsOwner,
                    #[codec(index = 7)]
                    NotIpfsEditor,
                    #[codec(index = 8)]
                    NotIpfsReader,
                    #[codec(index = 9)]
                    SameAccount,
                    #[codec(index = 10)]
                    SameOwnershipLayer,
                    #[codec(index = 11)]
                    IpfsAlreadyOwned,
                    #[codec(index = 12)]
                    CantCreateRequest,
                    #[codec(index = 13)]
                    RequestTimeout,
                    #[codec(index = 14)]
                    RequestFailed,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    QueuedDataToAdd(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 1)]
                    QueuedDataToCat(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 2)]
                    PublishedIdentity(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 3)]
                    Created(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                    #[codec(index = 4)]
                    PriceSet(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        ::core::option::Option<::core::primitive::u128>,
                    ),
                    #[codec(index = 5)]
                    Transferred(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                    #[codec(index = 6)]
                    Bought(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 7)]
                    AddOwner(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 8)]
                    RemoveOwner(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 9)]
                    ChangeOwnershipLayer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 10)]
                    WriteIpfsAsset(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                    #[codec(index = 11)]
                    ReadIpfsAsset(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                    #[codec(index = 12)]
                    DeleteIpfsAsset(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 13)]
                    AddPin(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 14)]
                    UnpinIpfsAsset(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Ipfs {
                    pub cid_addr: ::std::vec::Vec<::core::primitive::u8>,
                    pub gateway_url: ::std::vec::Vec<::core::primitive::u8>,
                    pub owners: ::subxt::KeyedVec<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_ipfs::pallet::OwnershipLayer,
                    >,
                    pub pinned: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum OwnershipLayer {
                    #[codec(index = 0)]
                    Owner,
                    #[codec(index = 1)]
                    Editor,
                    #[codec(index = 2)]
                    Reader,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum DataCommand<_0> {
                #[codec(index = 0)]
                AddBytes(
                    runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    ::std::vec::Vec<::core::primitive::u8>,
                    _0,
                    ::core::primitive::bool,
                ),
                #[codec(index = 1)]
                AddBytesRaw(
                    runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    ::std::vec::Vec<::core::primitive::u8>,
                    _0,
                    ::core::primitive::bool,
                ),
                #[codec(index = 2)]
                CatBytes(
                    runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    ::std::vec::Vec<::core::primitive::u8>,
                    _0,
                ),
                #[codec(index = 3)]
                InsertPin(
                    runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    ::std::vec::Vec<::core::primitive::u8>,
                    _0,
                    ::core::primitive::bool,
                ),
                #[codec(index = 4)]
                RemovePin(
                    runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    ::std::vec::Vec<::core::primitive::u8>,
                    _0,
                    ::core::primitive::bool,
                ),
                #[codec(index = 5)]
                RemoveBlock(
                    runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    ::std::vec::Vec<::core::primitive::u8>,
                    _0,
                ),
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    schedule_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 3)]
                    cancel_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 5)]
                    schedule_named_after {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    FailedToSchedule,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    TargetBlockNumberInPast,
                    #[codec(index = 3)]
                    RescheduleNoChange,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Scheduled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 1)]
                    Canceled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 2)]
                    Dispatched(
                        (::core::primitive::u32, ::core::primitive::u32),
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1,
                #[codec(index = 1)]
                V2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ScheduledV2<_0, _1, _2, _3> {
                pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub priority: ::core::primitive::u8,
                pub call: _0,
                pub maybe_periodic: ::core::option::Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid(::core::result::Result<(), runtime_types::sp_runtime::DispatchError>),
                    #[codec(index = 1)]
                    KeyChanged(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 2)]
                    SudoAsDone(
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod changes_trie {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: ::core::primitive::u32,
                    pub digest_levels: ::core::primitive::u32,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum ChangesTrieSignal {
                        #[codec(index = 0)]
                        NewConfiguration(
                            ::core::option::Option<
                                runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                            >,
                        ),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Digest<_0> {
                        pub logs: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem<_0>,
                        >,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum DigestItem<_0> {
                        #[codec(index = 2)]
                        ChangesTrieRoot(_0),
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 7)]
                        ChangesTrieSignal(
                            runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
        fn module_error_indices(&self) -> Option<(u8, u8)> {
            if let &Self::Module { index, error } = self {
                Some((index, error))
            } else {
                None
            }
        }
    }
    #[doc = r" The default storage entry from which to fetch an account nonce, required for"]
    #[doc = r" constructing a transaction."]
    pub enum DefaultAccountData {}
    impl ::subxt::AccountData for DefaultAccountData {
        type StorageEntry = self::system::storage::AccountOwned;
        type AccountId = ::subxt::sp_core::crypto::AccountId32;
        type Index = ::core::primitive::u32;
        fn storage_entry(account_id: Self::AccountId) -> Self::StorageEntry {
            self::system::storage::AccountOwned(account_id)
        }
        fn nonce(result: &<Self::StorageEntry as ::subxt::StorageEntry>::Value) -> Self::Index {
            result.nonce
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X, A = DefaultAccountData> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<T, X, A> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X, A> RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn constants(&'a self) -> ConstantsApi {
            ConstantsApi
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X, A> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
        pub fn events(&'a self) -> EventsApi<'a, T> {
            EventsApi {
                client: &self.client,
            }
        }
    }
    pub struct EventsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> EventsApi<'a, T> {
        pub async fn at(
            &self,
            block_hash: T::Hash,
        ) -> Result<::subxt::events::Events<'a, T, Event>, ::subxt::BasicError> {
            ::subxt::events::at::<T, Event>(self.client, block_hash).await
        }
        pub async fn subscribe(
            &self,
        ) -> Result<::subxt::events::EventSubscription<'a, T, Event>, ::subxt::BasicError> {
            ::subxt::events::subscribe::<T, Event>(self.client).await
        }
        pub async fn subscribe_finalized(
            &self,
        ) -> Result<::subxt::events::EventSubscription<'a, T, Event>, ::subxt::BasicError> {
            ::subxt::events::subscribe_finalized::<T, Event>(self.client).await
        }
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn ipfs(&self) -> ipfs::constants::ConstantsApi {
            ipfs::constants::ConstantsApi
        }
        pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
            scheduler::constants::ConstantsApi
        }
        pub fn assets(&self) -> assets::constants::ConstantsApi {
            assets::constants::ConstantsApi
        }
        pub fn contracts(&self) -> contracts::constants::ConstantsApi {
            contracts::constants::ConstantsApi
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn ipfs(&self) -> ipfs::storage::StorageApi<'a, T> {
            ipfs::storage::StorageApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
            scheduler::storage::StorageApi::new(self.client)
        }
        pub fn assets(&self) -> assets::storage::StorageApi<'a, T> {
            assets::storage::StorageApi::new(self.client)
        }
        pub fn contracts(&self) -> contracts::storage::StorageApi<'a, T> {
            contracts::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<'a, T, X, A> TransactionApi<'a, T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X, A> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X, A> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X, A> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X, A> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X, A> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn ipfs(&self) -> ipfs::calls::TransactionApi<'a, T, X, A> {
            ipfs::calls::TransactionApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T, X, A> {
            scheduler::calls::TransactionApi::new(self.client)
        }
        pub fn contracts(&self) -> contracts::calls::TransactionApi<'a, T, X, A> {
            contracts::calls::TransactionApi::new(self.client)
        }
    }
}
