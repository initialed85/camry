# coding: utf-8

"""
    Djangolang

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.vec2 import Vec2

class TestVec2(unittest.TestCase):
    """Vec2 unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> Vec2:
        """Test Vec2
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `Vec2`
        """
        model = Vec2()
        if include_optional:
            return Vec2(
                x = 1.337,
                y = 1.337
            )
        else:
            return Vec2(
        )
        """

    def testVec2(self):
        """Test Vec2"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()