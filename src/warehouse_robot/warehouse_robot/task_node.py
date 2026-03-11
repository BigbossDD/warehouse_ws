import rclpy
from rclpy.node import Node
from example_interfaces.srv import Trigger

class TaskNode(Node):

    def __init__(self):
        super().__init__('task_node')

        self.srv = self.create_service(
            Trigger,
            'request_delivery',
            self.delivery_callback
        )

        self.get_logger().info("Task Node Ready")

    def delivery_callback(self, request, response):

        self.get_logger().info("Delivery request received!")

        response.success = True
        response.message = "Robot is moving to the requested destination."

        return response


def main(args=None):

    rclpy.init(args=args)

    node = TaskNode()

    rclpy.spin(node)

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()