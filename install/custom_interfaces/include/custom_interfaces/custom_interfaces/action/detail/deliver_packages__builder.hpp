// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from custom_interfaces:action/DeliverPackages.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_INTERFACES__ACTION__DETAIL__DELIVER_PACKAGES__BUILDER_HPP_
#define CUSTOM_INTERFACES__ACTION__DETAIL__DELIVER_PACKAGES__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "custom_interfaces/action/detail/deliver_packages__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_Goal_destination
{
public:
  Init_DeliverPackages_Goal_destination()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::custom_interfaces::action::DeliverPackages_Goal destination(::custom_interfaces::action::DeliverPackages_Goal::_destination_type arg)
  {
    msg_.destination = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_Goal>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_Goal_destination();
}

}  // namespace custom_interfaces


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_Result_message
{
public:
  explicit Init_DeliverPackages_Result_message(::custom_interfaces::action::DeliverPackages_Result & msg)
  : msg_(msg)
  {}
  ::custom_interfaces::action::DeliverPackages_Result message(::custom_interfaces::action::DeliverPackages_Result::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_Result msg_;
};

class Init_DeliverPackages_Result_success
{
public:
  Init_DeliverPackages_Result_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DeliverPackages_Result_message success(::custom_interfaces::action::DeliverPackages_Result::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_DeliverPackages_Result_message(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_Result>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_Result_success();
}

}  // namespace custom_interfaces


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_Feedback_distance_remaining
{
public:
  Init_DeliverPackages_Feedback_distance_remaining()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::custom_interfaces::action::DeliverPackages_Feedback distance_remaining(::custom_interfaces::action::DeliverPackages_Feedback::_distance_remaining_type arg)
  {
    msg_.distance_remaining = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_Feedback>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_Feedback_distance_remaining();
}

}  // namespace custom_interfaces


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_SendGoal_Request_goal
{
public:
  explicit Init_DeliverPackages_SendGoal_Request_goal(::custom_interfaces::action::DeliverPackages_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::custom_interfaces::action::DeliverPackages_SendGoal_Request goal(::custom_interfaces::action::DeliverPackages_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_SendGoal_Request msg_;
};

class Init_DeliverPackages_SendGoal_Request_goal_id
{
public:
  Init_DeliverPackages_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DeliverPackages_SendGoal_Request_goal goal_id(::custom_interfaces::action::DeliverPackages_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_DeliverPackages_SendGoal_Request_goal(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_SendGoal_Request>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_SendGoal_Request_goal_id();
}

}  // namespace custom_interfaces


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_SendGoal_Response_stamp
{
public:
  explicit Init_DeliverPackages_SendGoal_Response_stamp(::custom_interfaces::action::DeliverPackages_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::custom_interfaces::action::DeliverPackages_SendGoal_Response stamp(::custom_interfaces::action::DeliverPackages_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_SendGoal_Response msg_;
};

class Init_DeliverPackages_SendGoal_Response_accepted
{
public:
  Init_DeliverPackages_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DeliverPackages_SendGoal_Response_stamp accepted(::custom_interfaces::action::DeliverPackages_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_DeliverPackages_SendGoal_Response_stamp(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_SendGoal_Response>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_SendGoal_Response_accepted();
}

}  // namespace custom_interfaces


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_GetResult_Request_goal_id
{
public:
  Init_DeliverPackages_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::custom_interfaces::action::DeliverPackages_GetResult_Request goal_id(::custom_interfaces::action::DeliverPackages_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_GetResult_Request>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_GetResult_Request_goal_id();
}

}  // namespace custom_interfaces


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_GetResult_Response_result
{
public:
  explicit Init_DeliverPackages_GetResult_Response_result(::custom_interfaces::action::DeliverPackages_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::custom_interfaces::action::DeliverPackages_GetResult_Response result(::custom_interfaces::action::DeliverPackages_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_GetResult_Response msg_;
};

class Init_DeliverPackages_GetResult_Response_status
{
public:
  Init_DeliverPackages_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DeliverPackages_GetResult_Response_result status(::custom_interfaces::action::DeliverPackages_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_DeliverPackages_GetResult_Response_result(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_GetResult_Response>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_GetResult_Response_status();
}

}  // namespace custom_interfaces


namespace custom_interfaces
{

namespace action
{

namespace builder
{

class Init_DeliverPackages_FeedbackMessage_feedback
{
public:
  explicit Init_DeliverPackages_FeedbackMessage_feedback(::custom_interfaces::action::DeliverPackages_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::custom_interfaces::action::DeliverPackages_FeedbackMessage feedback(::custom_interfaces::action::DeliverPackages_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_FeedbackMessage msg_;
};

class Init_DeliverPackages_FeedbackMessage_goal_id
{
public:
  Init_DeliverPackages_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DeliverPackages_FeedbackMessage_feedback goal_id(::custom_interfaces::action::DeliverPackages_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_DeliverPackages_FeedbackMessage_feedback(msg_);
  }

private:
  ::custom_interfaces::action::DeliverPackages_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::action::DeliverPackages_FeedbackMessage>()
{
  return custom_interfaces::action::builder::Init_DeliverPackages_FeedbackMessage_goal_id();
}

}  // namespace custom_interfaces

#endif  // CUSTOM_INTERFACES__ACTION__DETAIL__DELIVER_PACKAGES__BUILDER_HPP_
