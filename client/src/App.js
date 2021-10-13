import React, { useState } from 'react'
import { Wallet } from './Wallet'
import { CreateLoan } from './CreateLoan'
import { Trade } from './Trade'

export const App = () => {    
    const [stateAddress, setStateAddress] = useState()
    const [authorityAddress, setAuthorityAddress] = useState()

    
    // if loan created, trade buttons for long / short
    // show interest rate, lamports received, 
    // current prices of long / short tokens and circulation
    
    const initialized = !stateAddress && !authorityAddress
    return (
        <div>
            <Wallet />
            {initialized ? 
                <Trade stateAddress={stateAddress} authorityAddress={authorityAddress} /> : 
                <CreateLoan setStateAddress={setStateAddress} setAuthorityAddress={setAuthorityAddress} />}
        </div>
    )
}