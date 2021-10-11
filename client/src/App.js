import React, { useState, setState } from 'react'
import { Wallet } from './Wallet'
import { CreateLoan } from './CreateLoan'
import { Trade } from './Trade'
import { PublicKey } from '@solana/web3.js'
import { utf8 } from '@project-serum/anchor/dist/cjs/utils/bytes'

export const App = () => {
    // if loan not created, create loan button
    // initialize loan
    const stateAddress = useState([])
    const authorityAddress = useState()

    if (!stateAddress) {
        const [stateAddress, bump] = await PublicKey.findProgramAddress(
            [Buffer.from(utf8.encode('state_v1'))],
            program.programId
       );

       setState()

    }

    // find it here and set it?

    // if loan created, trade buttons for long / short
    // show interest rate, lamports received, 
    // current prices of long / short tokens and circulation
    
    const initialized = !stateAddress && !authorityAddress
    return (
        <div>
            <Wallet />
            {initialized ? 
                <Trade stateAddress={stateAddress} authorityAddress={authorityAddress} /> : 
                <CreateLoan stateAddress={stateAddress} authorityAddress={authorityAddress} />}
        </div>
    )
}