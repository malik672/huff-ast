// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Crowdfunding {
    address public projectOwner;
    uint public fundingGoal;
    uint public deadline;
    uint public totalFunds;

    mapping(address => uint) public contributions;

    enum State { Fundraising, Successful, Expired, Refunding }
    State public currentState = State.Fundraising;

    event ProjectFunded(address indexed contributor, uint amount);
    event FundingGoalReached(uint totalFunds);
    event FundingExpired();
    event FundsRefunded(address indexed contributor, uint amount);

    modifier onlyProjectOwner() {
        require(msg.sender == projectOwner, "Only the project owner can call this function");
        _;
    }

    modifier inState(State _state) {
        require(currentState == _state, "Invalid state for this operation");
        _;
    }

    constructor(address _projectOwner, uint _fundingGoal, uint _durationDays) {
        projectOwner = _projectOwner;
        fundingGoal = _fundingGoal * 1 ether; // Convert to wei
        deadline = block.timestamp + (_durationDays * 1 days);
    }

    function contribute() external payable inState(State.Fundraising) {
        require(block.timestamp < deadline, "Funding period has ended");

        contributions[msg.sender] += msg.value;
        totalFunds += msg.value;

        emit ProjectFunded(msg.sender, msg.value);

        if (totalFunds >= fundingGoal) {
            currentState = State.Successful;
            emit FundingGoalReached(totalFunds);
        }
    }

    function withdraw() external onlyProjectOwner inState(State.Successful) {
        payable(projectOwner).transfer(totalFunds);
    }

    function refund() external inState(State.Refunding) {
        uint amount = contributions[msg.sender];
        require(amount > 0, "You have no funds to refund");

        contributions[msg.sender] = 0;
        payable(msg.sender).transfer(amount);

        emit FundsRefunded(msg.sender, amount);
    }

    function checkGoalReached() external {
        if (block.timestamp >= deadline && totalFunds < fundingGoal) {
            currentState = State.Expired;
            emit FundingExpired();
        } else if (totalFunds >= fundingGoal) {
            currentState = State.Successful;
            emit FundingGoalReached(totalFunds);
        }
    }

    function enableRefunds() external onlyProjectOwner inState(State.Fundraising) {
        require(block.timestamp >= deadline, "Refunds can only be enabled after the deadline");
        currentState = State.Refunding;
    }
}
