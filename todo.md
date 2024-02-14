To build a token staking platform for your new token on Solana using Rust and the Anchor framework, you should implement the following functionalities:

### Initialization
- **Initialize**: Set up the staking pool with parameters such as the admin address, start and end slots for staking periods, and any other initial configurations.

### Staking Operations
- **Stake**: Allow users to deposit tokens into the staking contract. This function should record the amount staked and the time of staking to calculate rewards.
- **Unstake**: Enable users to withdraw their staked tokens. This may involve a cooldown period or instant unstaking depending on your platform's rules.
- **Claim Reward**: Provide a function for users to claim their staking rewards. The rewards could be calculated based on the amount of time the tokens were staked and the staking rate.

### Administrative Functions
- **Update Staking Parameters**: Functions for the admin to update staking parameters, such as reward rates or periods.
- **Distribute Rewards**: A mechanism for distributing staking rewards, which could be automated or initiated by the admin.

### Security Measures
- **Pause/Resume**: Implement the ability to pause and resume staking in case of emergencies or maintenance.

### Token Allocation
- Ensure that the smart contract handles the token allocation as specified: 40% to the LP, 10% to the devs, and 50% to the staking platform. This will involve setting up the initial distribution and possibly creating separate pools or accounts for each category.

### Additional Considerations
- **Integration with Frontend**: You will need to create a frontend interface that interacts with your smart contract to allow users to stake and unstake tokens, and claim rewards.
- **Testing**: Write tests for your smart contract to ensure that all functionalities work as expected and that there are no security vulnerabilities.
- **Documentation**: Provide clear documentation for users and developers interacting with your staking platform.

[23:35, 09/02/2024] Tiago: Depende do total staked
[23:35, 09/02/2024] Tiago: Imagina podes decidir aue vais escoar x% do supply total diariamente
[23:36, 09/02/2024] Tiago: Ou seja vamos dizer
[23:36, 09/02/2024] Tiago: Há 50% do supply no SC
[23:36, 09/02/2024] Tiago: E aueres distribuir 1% daily
[23:36, 09/02/2024] Tiago: O teu apy vai depender do pessoal q está na pool
[23:36, 09/02/2024] Tiago: Se só há 1 pessoa o teu apy é bué alto
