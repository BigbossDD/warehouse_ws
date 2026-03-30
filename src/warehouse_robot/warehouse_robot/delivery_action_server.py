import rclpy
from rclpy.node import Node
from rclpy.action import ActionServer
from custom_interfaces.action import DeliverPackages
import time

class DeliveryActionServer(Node):

    def __init__(self):
        super().__init__('delivery_action_server')

        self._action_server = ActionServer(
            self,
            DeliverPackage,
            'deliver_package',
            self.execute_callback
        )

        self.get_logger().info('Delivery Action Server is ready!')

    def execute_callback(self, goal_handle):
        self.get_logger().info(f'Received goal: Go to {goal_handle.request.destination}')

        # Simulate driving — 10 steps of "moving"
        total_distance = 10.0

        for i in range(10):
            remaining = total_distance - (i + 1)

            # Send feedback every step
            feedback_msg = DeliverPackage.Feedback()
            feedback_msg.distance_remaining = remaining
            goal_handle.publish_feedback(feedback_msg)

            self.get_logger().info(f'Feedback: Distance remaining = {remaining}m')
            time.sleep(1)  # wait 1 second between steps

        # Goal completed
        goal_handle.succeed()

        result = DeliverPackage.Result()
        result.success = True
        result.message = f'Successfully delivered to {goal_handle.request.destination}!'

        self.get_logger().info('Goal succeeded!')
        return result


def main(args=None):
    rclpy.init(args=args)
    node = DeliveryActionServer()
    rclpy.spin(node)
    rclpy.shutdown()

if __name__ == '__main__':
    main()