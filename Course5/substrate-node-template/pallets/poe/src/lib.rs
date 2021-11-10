//
//@author Bayek
//@dev This is a substrate poe module.
//

#![cfg_attr(not(feature = "std"), no_std)]
// 把 pallet 模块中定义的功能组件都给暴露出来，使得能在 Runtime 中进行引用

/// A module for proof of existence
pub use pallet::*;
// 把Pallet, Call, Storage, Event<T>暴露出来

// 使用pallet support 下面的 pallet 宏
#[frame_support::pallet]
pub mod pallet {
    // 引入对应的依赖
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        // 可调函数的一个返回结果
        pallet_prelude::*,
        // 包含一些 Runtime 开发所需要的宏
        sp_std::vec::Vec
    };
    use frame_system::pallet_prelude::*;
    // 引入了系统模块 pallet_prelude 依赖的一些常用数据和类型信息
    //use sp_std::vec::Vec;
    // 使用sp_std，添加vec路径

    // 定义模块配置接口
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // 定义pallet结构体来承担我们的功能模块
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap <
        // 定义存储单元，用来存储存证，其类型为 StorageMap
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber)
        // BlockNumber 表示存入存证的区块
    >;

    // 用 generate_deposit宏 可以生成一个帮助性的方法，方便进行 event 的触发
    #[pallet::event]
    //#[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // 定义一个 event 的枚举类型
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        // 利用 pallet::error宏 来定义一个 error 信息
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // 利用 pallet::hooks宏 来定义某些可以在区块特殊时期执行的函数
    
    // 定义可调用函数
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // 创建存证的功能实现，定义创建存证的可调用函数
        #[pallet::weight(0)] 
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // 定义创建存证的可调运函数，origin 表示交易的发送方，claim 表示存证的哈希值

            let sender = ensure_signed(origin)?;
            // 校验发送方

            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
            // 校验存证的方法

            Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Pallet::<T>::block_number()));
            // 存储记录

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            // 插入成功触发事件

            Ok(().into())
            // 返回一个 result 类型，并且进行转换
        }

        // 撤销存证的功能实现，定义撤销存证的可调用函数
        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {

            let sender = ensure_signed(origin)?;
            // 校验当前的交易发送方是一个已经签名的交易

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            // 校验当前的存储里面存在这样一个值

            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            // 校验当前交易的发送方是不是 proof的owner ，如果是，则允许撤销，如果不是，则发出错误 NotClaimOwner

            Proofs::<T>::remove(&claim);
            // 做完校验以后，可以调运 Proofs上的 remove 方法来删除一条记录

            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            // 触发事件 ClaimRevoked

            Ok(().into())
            // 返回一个 result 类型，并且进行转换
        }
    }
    
}