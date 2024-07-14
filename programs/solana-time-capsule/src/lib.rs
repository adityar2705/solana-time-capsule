use anchor_lang::prelude::*;

//Program ID of the time capsule smart contract : 9W3vLonoNDoDEgT26me4Do3Srw3FGqcpdgW8GGAT62C4
declare_id!("9W3vLonoNDoDEgT26me4Do3Srw3FGqcpdgW8GGAT62C4");

#[program]
pub mod solana_time_capsule {
    use super::*;

    //function to initialize the time capsule
    pub fn initialize(ctx: Context<Initialize>, data : String) -> Result<()>{
        let base_account = &mut ctx.accounts.base_account;
        let data_copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(data_copy);
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    //function to update the time capsule
    pub fn update(ctx : Context<Update>, data : String) -> Result<()>{
        let base_account = &mut ctx.accounts.base_account;
        let data_copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(data_copy);
        Ok(())
    }
}

//struct for context for the initialization of the time capsule
#[derive(Accounts)]
pub struct Initialize<'info>{
    //creation of the time capsule PDA
    #[account(init,payer = user,space = 64 + 64)]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info,System>
}

//struct for context for the updating of the time capsule 
#[derive(Accounts)]
pub struct Update<'info>{
    //since did not pass any seeds there is only one PDA for this program as of now so we are referencing that
    #[account(mut)]
    pub base_account : Account<'info,BaseAccount>,

    #[account(mut)]
    pub user : Signer<'info>
}

//struct for the time capsule PDA storage 
#[account]
pub struct BaseAccount{
    pub data : String,
    pub data_list : Vec<String>
}

