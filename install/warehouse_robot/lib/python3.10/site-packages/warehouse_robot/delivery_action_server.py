import time
import rclpy
from rclpy.action import ActionServer, CancelResponse, GoalResponse
from rclpy.node import Node
from rclpy.executors import MultiThreadedExecutor
from rclpy.callback_groups import ReentrantCallbackGroup

from custom_interfaces.action import DeliverPackages


class DeliveryActionServer(Node):

    def __init__(self):
        super().__init__('delivery_action_server')

        self._callback_group = ReentrantCallbackGroup()

        self._action_server = ActionServer(
            self,
            DeliverPackages,
            'deliver_package',
            execute_callback=self.execute_callback,
            goal_callback=self.goal_callback,
            cancel_callback=self.cancel_callback,
            callback_group=self._callback_group
        )

        self.get_logger().info("<Delivery Action Server is ready...>")

    ############################################################
    def goal_callback(self, goal_request):
        self.get_logger().info(f"New delivery request to: {goal_request.destination}")

        if goal_request.destination == "":
            self.get_logger().warn("Empty destination rejected")
            return GoalResponse.REJECT

        return GoalResponse.ACCEPT

    # ##############################!##########
    
    def cancel_callback(self, goal_handle):
        self.get_logger().info("cancel requests received")
        return CancelResponse.ACCEPT

    # ##############################!##########
   
    async def execute_callback(self, goal_handle):
        self.get_logger().info("Starting delivery...")

        feedback_msg = DeliverPackages.Feedback()
        result = DeliverPackages.Result()

        
        distance = 10.0 #NOTE this for testing remove later

        while distance > 0.0:

            
            if goal_handle.is_cancel_requested:
                goal_handle.canceled()
                result.success = False
                result.message = "Delivery canceled"
                self.get_logger().info("Delivery canceled")
                return result

            ################################
            feedback_msg.distance_remaining = distance
            self.get_logger().info(f"Distance remaining: {distance:.2f} m")
            goal_handle.publish_feedback(feedback_msg)#NOTE

            #NOTE this is to just to test the node 
            time.sleep(1.0)
            distance -= 1.0

            #  # # # # #  # # # # # ## 
        goal_handle.succeed()
        result.success = True
        result.message = "Package delivered successfully"

        self.get_logger().info("Delivery completed!")

        return result

################################################################################
def main(args=None):
    rclpy.init(args=args)

    node = DeliveryActionServer()

    executor = MultiThreadedExecutor()
    rclpy.spin(node, executor=executor)

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()