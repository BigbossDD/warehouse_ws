import rclpy
from rclpy.node import Node
from example_interfaces.srv import Trigger
import time 
import random
class TaskNode(Node):

    def __init__(self):
        super().__init__('task_node')
        self.get_logger().info("task node Ready awaiting orders")

        self.srv = self.create_service(
            Trigger,
            'request_delivery',
            self.delivery_callback
        )

        

    def delivery_callback(self, request, response):

        self.get_logger().info("request received")

        time.sleep(5)
        response.success = True
        response.message = "~~robot is on the move~~"

        return response
def main(args=None):

    rclpy.init(args=args)
#
    node = TaskNode()
#
    rclpy.spin(node)
#
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    print(' task node is on')
    main(None)
    print('task nodes is off')
