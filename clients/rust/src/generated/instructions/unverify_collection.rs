//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct UnverifyCollection {
    /// Metadata account
    pub metadata: solana_program::pubkey::Pubkey,
    /// Collection Authority
    pub collection_authority: solana_program::pubkey::Pubkey,
    /// Mint of the Collection
    pub collection_mint: solana_program::pubkey::Pubkey,
    /// Metadata Account of the Collection
    pub collection: solana_program::pubkey::Pubkey,
    /// MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: solana_program::pubkey::Pubkey,
    /// Collection Authority Record PDA
    pub collection_authority_record: Option<solana_program::pubkey::Pubkey>,
}

impl UnverifyCollection {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[super::InstructionAccount],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.collection_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection_master_edition_account,
            false,
        ));
        if let Some(collection_authority_record) = self.collection_authority_record {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                collection_authority_record,
                false,
            ));
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| accounts.push(remaining_account.to_account_meta()));
        let data = UnverifyCollectionInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct UnverifyCollectionInstructionData {
    discriminator: u8,
}

impl UnverifyCollectionInstructionData {
    fn new() -> Self {
        Self { discriminator: 22 }
    }
}

/// Instruction builder.
#[derive(Default)]
pub struct UnverifyCollectionBuilder {
    metadata: Option<solana_program::pubkey::Pubkey>,
    collection_authority: Option<solana_program::pubkey::Pubkey>,
    collection_mint: Option<solana_program::pubkey::Pubkey>,
    collection: Option<solana_program::pubkey::Pubkey>,
    collection_master_edition_account: Option<solana_program::pubkey::Pubkey>,
    collection_authority_record: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<super::InstructionAccount>,
}

impl UnverifyCollectionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Metadata account
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    /// Collection Authority
    #[inline(always)]
    pub fn collection_authority(
        &mut self,
        collection_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_authority = Some(collection_authority);
        self
    }
    /// Mint of the Collection
    #[inline(always)]
    pub fn collection_mint(
        &mut self,
        collection_mint: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_mint = Some(collection_mint);
        self
    }
    /// Metadata Account of the Collection
    #[inline(always)]
    pub fn collection(&mut self, collection: solana_program::pubkey::Pubkey) -> &mut Self {
        self.collection = Some(collection);
        self
    }
    /// MasterEdition2 Account of the Collection Token
    #[inline(always)]
    pub fn collection_master_edition_account(
        &mut self,
        collection_master_edition_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_master_edition_account = Some(collection_master_edition_account);
        self
    }
    /// `[optional account]`
    /// Collection Authority Record PDA
    #[inline(always)]
    pub fn collection_authority_record(
        &mut self,
        collection_authority_record: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.collection_authority_record = collection_authority_record;
        self
    }
    #[inline(always)]
    pub fn add_remaining_account(&mut self, account: super::InstructionAccount) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    #[inline(always)]
    pub fn add_remaining_accounts(&mut self, accounts: &[super::InstructionAccount]) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = UnverifyCollection {
            metadata: self.metadata.expect("metadata is not set"),
            collection_authority: self
                .collection_authority
                .expect("collection_authority is not set"),
            collection_mint: self.collection_mint.expect("collection_mint is not set"),
            collection: self.collection.expect("collection is not set"),
            collection_master_edition_account: self
                .collection_master_edition_account
                .expect("collection_master_edition_account is not set"),
            collection_authority_record: self.collection_authority_record,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `unverify_collection` CPI accounts.
pub struct UnverifyCollectionCpiAccounts<'a> {
    /// Metadata account
    pub metadata: &'a solana_program::account_info::AccountInfo<'a>,
    /// Collection Authority
    pub collection_authority: &'a solana_program::account_info::AccountInfo<'a>,
    /// Mint of the Collection
    pub collection_mint: &'a solana_program::account_info::AccountInfo<'a>,
    /// Metadata Account of the Collection
    pub collection: &'a solana_program::account_info::AccountInfo<'a>,
    /// MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'a solana_program::account_info::AccountInfo<'a>,
    /// Collection Authority Record PDA
    pub collection_authority_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
}

/// `unverify_collection` CPI instruction.
pub struct UnverifyCollectionCpi<'a> {
    /// The program to invoke.
    pub __program: &'a solana_program::account_info::AccountInfo<'a>,
    /// Metadata account
    pub metadata: &'a solana_program::account_info::AccountInfo<'a>,
    /// Collection Authority
    pub collection_authority: &'a solana_program::account_info::AccountInfo<'a>,
    /// Mint of the Collection
    pub collection_mint: &'a solana_program::account_info::AccountInfo<'a>,
    /// Metadata Account of the Collection
    pub collection: &'a solana_program::account_info::AccountInfo<'a>,
    /// MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'a solana_program::account_info::AccountInfo<'a>,
    /// Collection Authority Record PDA
    pub collection_authority_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
}

impl<'a> UnverifyCollectionCpi<'a> {
    pub fn new(
        program: &'a solana_program::account_info::AccountInfo<'a>,
        accounts: UnverifyCollectionCpiAccounts<'a>,
    ) -> Self {
        Self {
            __program: program,
            metadata: accounts.metadata,
            collection_authority: accounts.collection_authority,
            collection_mint: accounts.collection_mint,
            collection: accounts.collection,
            collection_master_edition_account: accounts.collection_master_edition_account,
            collection_authority_record: accounts.collection_authority_record,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[super::InstructionAccountInfo<'a>],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[super::InstructionAccountInfo<'a>],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.collection_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection_master_edition_account.key,
            false,
        ));
        if let Some(collection_authority_record) = self.collection_authority_record {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *collection_authority_record.key,
                false,
            ));
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| accounts.push(remaining_account.to_account_meta()));
        let data = UnverifyCollectionInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.metadata.clone());
        account_infos.push(self.collection_authority.clone());
        account_infos.push(self.collection_mint.clone());
        account_infos.push(self.collection.clone());
        account_infos.push(self.collection_master_edition_account.clone());
        if let Some(collection_authority_record) = self.collection_authority_record {
            account_infos.push(collection_authority_record.clone());
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            account_infos.push(remaining_account.account_info().clone())
        });

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `unverify_collection` CPI instruction builder.
pub struct UnverifyCollectionCpiBuilder<'a> {
    instruction: Box<UnverifyCollectionCpiBuilderInstruction<'a>>,
}

impl<'a> UnverifyCollectionCpiBuilder<'a> {
    pub fn new(program: &'a solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UnverifyCollectionCpiBuilderInstruction {
            __program: program,
            metadata: None,
            collection_authority: None,
            collection_mint: None,
            collection: None,
            collection_master_edition_account: None,
            collection_authority_record: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Metadata account
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    /// Collection Authority
    #[inline(always)]
    pub fn collection_authority(
        &mut self,
        collection_authority: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_authority = Some(collection_authority);
        self
    }
    /// Mint of the Collection
    #[inline(always)]
    pub fn collection_mint(
        &mut self,
        collection_mint: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_mint = Some(collection_mint);
        self
    }
    /// Metadata Account of the Collection
    #[inline(always)]
    pub fn collection(
        &mut self,
        collection: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection = Some(collection);
        self
    }
    /// MasterEdition2 Account of the Collection Token
    #[inline(always)]
    pub fn collection_master_edition_account(
        &mut self,
        collection_master_edition_account: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_master_edition_account =
            Some(collection_master_edition_account);
        self
    }
    /// `[optional account]`
    /// Collection Authority Record PDA
    #[inline(always)]
    pub fn collection_authority_record(
        &mut self,
        collection_authority_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.collection_authority_record = collection_authority_record;
        self
    }
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: super::InstructionAccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.__remaining_accounts.push(account);
        self
    }
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[super::InstructionAccountInfo<'a>],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = UnverifyCollectionCpi {
            __program: self.instruction.__program,

            metadata: self.instruction.metadata.expect("metadata is not set"),

            collection_authority: self
                .instruction
                .collection_authority
                .expect("collection_authority is not set"),

            collection_mint: self
                .instruction
                .collection_mint
                .expect("collection_mint is not set"),

            collection: self.instruction.collection.expect("collection is not set"),

            collection_master_edition_account: self
                .instruction
                .collection_master_edition_account
                .expect("collection_master_edition_account is not set"),

            collection_authority_record: self.instruction.collection_authority_record,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct UnverifyCollectionCpiBuilderInstruction<'a> {
    __program: &'a solana_program::account_info::AccountInfo<'a>,
    metadata: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    collection_authority: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    collection_mint: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    collection: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    collection_master_edition_account: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    collection_authority_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    __remaining_accounts: Vec<super::InstructionAccountInfo<'a>>,
}
